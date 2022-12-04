mod util;
mod vec3;
mod color;
mod ray;
mod hittable;
mod sphere;
mod moving_sphere;
mod camera;
mod material;
mod aabb;
mod bvh;
mod texture;
mod perlin;
mod aarect;
mod cube;

use std::f64::INFINITY;
use aarect::{XYRect, YZRect, XZRect};
use bvh::BVH;
use indicatif::ProgressBar;
use material::DiffuseLight;
use moving_sphere::MovingSphere;
use rayon::prelude::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use texture::{CheckerTexture, NoiseTexture, ImageTexture};

use crate::camera::Camera;
use crate::hittable::Hittable;
use crate::material::{Lambertian, Metal, Dialectric};
use crate::sphere::Sphere;
use crate::util::{random_double, random_double_in_range, divide_into_parts};
use crate::vec3::{Point3, Color, unit_vector, Vec3};
use crate::color::write_color;
use crate::ray::Ray;

fn render_image(cam: &Camera, world: &dyn Hittable, background: &Color, image_width: i32, image_height: i32, samples_per_pixel: i32, max_depth: i32, bar: &ProgressBar) -> Vec<Color> {
  let mut output : Vec<Color> = Vec::new();
  for j in (0..image_height).rev() {
    for i in 0..image_width {
      let mut pixel_color = Color::new(0.0, 0.0, 0.0);
      for _ in 0..samples_per_pixel {
        let u = (i as f64 + random_double()) / (image_width as f64 - 1.0);
        let v = (j as f64 + random_double()) / (image_height as f64 - 1.0);
        let r = cam.get_ray(u, v);
        pixel_color += ray_color(&r, background, world, max_depth);
      }
      output.push(pixel_color);
      bar.inc(1);
    }
  }
  output
}

fn ray_color(r: &Ray, background: &Color, world: &dyn Hittable, depth: i32) -> Color {
  // If we've exceeded the ray bounce limit, no more light is gathered.
  if depth <= 0 { return Color::new(0.0, 0.0, 0.0) };

  match world.hit(r, 0.001, INFINITY) {
    Some(rec) => {
      let emitted = rec.material.emitted(rec.u, rec.v, &rec.p);
      match rec.material.scatter(r, &rec) {
        Some((attenuation, scattered)) => {
          emitted + attenuation * ray_color(&scattered, background, world, depth - 1)
        },
        None => emitted
      }
    },
    None => *background
  }
}

fn random_scene() -> BVH {
  let mut objects: Vec<Box<dyn Hittable>> = Vec::new();

  let checker = CheckerTexture::solid(Color::new(0.2,0.3,0.1), Color::new(0.9, 0.9, 0.9));
  objects.push(Box::new(Sphere::new(Point3::new(0.0,-1000.0,0.0), 1000.0, Lambertian::new(checker))));

  for a in -22..22 {
    for b in -22..22 {
      let choose_mat = random_double();
      let center = Point3::new(a as f64 + 0.9*random_double(), 0.2, b as f64 + 0.9*random_double());

      if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
        if choose_mat < 0.8 {
          // diffuse
          let albedo = Color::random() * Color::random();
          let material = Lambertian::solid(albedo);
          let center1 = center + Vec3::new(0.0, random_double_in_range(0.0, 0.5), 0.0);
          objects.push(Box::new(MovingSphere::new(center, center1, 0.0, 1.0, 0.2, material)));
        } else if choose_mat < 0.95 {
          // metal
          let albedo = Color::random_in_range(0.5, 1.0);
          let fuzz = random_double_in_range(0.0, 0.5);
          let material = Metal::solid(albedo, fuzz);
          objects.push(Box::new(Sphere::new(center, 0.2, material)));
        } else {
          // glass
          let material = Dialectric { ir: 1.5 };
          objects.push(Box::new(Sphere::new(center, 0.2, material)));
        }
      }
    }
  }

  let material1 = Dialectric { ir: 1.5 };
  objects.push(Box::new(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, material1)));

  let material2 = Lambertian::solid(Color::new(0.4, 0.2, 0.1));
  objects.push(Box::new(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, material2)));

  let material3 = Metal::solid(Color::new(0.7, 0.6, 0.5), 0.0);
  objects.push(Box::new(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, material3)));

  BVH::new(objects, 0.0, 1.0)
}

fn two_spheres() -> BVH {
  let mut objects: Vec<Box<dyn Hittable>> = Vec::new();

  let checker = CheckerTexture::solid(Color::new(0.2,0.3,0.1), Color::new(0.9, 0.9, 0.9));

  objects.push(Box::new(Sphere::new(Point3::new(0.0,-10.0,0.0), 10.0, Lambertian::new(checker))));
  objects.push(Box::new(Sphere::new(Point3::new(0.0,10.0,0.0), 10.0, Lambertian::new(checker))));

  BVH::new(objects, 0.0, 0.0)
}

fn two_perlin_spheres() -> BVH {
  let mut objects: Vec<Box<dyn Hittable>> = Vec::new();

  let pertext = NoiseTexture::new(4.0);

  objects.push(Box::new(Sphere::new(Point3::new(0.0,-1000.0,0.0), 1000.0, Lambertian::new(pertext))));
  objects.push(Box::new(Sphere::new(Point3::new(0.0,2.0,0.0), 2.0, Lambertian::new(pertext))));

  BVH::new(objects, 0.0, 0.0)
}

fn earth() -> BVH {
  let earth_texture = ImageTexture::new("earthmap.jpg");
  let earth_surface = Lambertian::new(earth_texture);
  let globe = Box::new(Sphere::new(Point3::zero(), 2.0, earth_surface));

  BVH::new(vec![globe], 0.0, 0.0)
}

fn simple_light() -> BVH {
  let mut objects: Vec<Box<dyn Hittable>> = Vec::new();

  let pertext = NoiseTexture::new(4.0);
  objects.push(Box::new(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, Lambertian::new(pertext))));
  objects.push(Box::new(Sphere::new(Point3::new(0.0, 2.0, 0.0), 2.0, Lambertian::new(pertext))));

  let difflight = DiffuseLight::solid(Color::new(4.0,4.0,4.0));
  objects.push(Box::new(XYRect::new(3.0, 5.0, 1.0, 3.0, -2.0, difflight)));
  objects.push(Box::new(Sphere::new(Point3::new(0.0, 7.0, 0.0), 2.0, difflight)));

  BVH::new(objects, 0.0, 0.0)
}

fn cornell_box() -> BVH {
  let mut objects: Vec<Box<dyn Hittable>> = Vec::new();

  let red = Lambertian::solid(Color::new(0.65, 0.05, 0.05));
  let white = Lambertian::solid(Color::new(0.73, 0.73, 0.73));
  let green = Lambertian::solid(Color::new(0.12, 0.45, 0.15));
  let light = DiffuseLight::solid(Color::new(15.0,15.0,15.0));

  objects.push(Box::new(YZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, green)));
  objects.push(Box::new(YZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red)));
  objects.push(Box::new(XZRect::new(213.0, 343.0, 227.0, 332.0, 554.0, light)));
  objects.push(Box::new(XZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, white)));
  objects.push(Box::new(XZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white)));
  objects.push(Box::new(XYRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white)));

  BVH::new(objects, 0.0, 0.0)
}

fn main() {
  // Image

  let aspect_ratio = 1.0;
  let image_width = 400;
  let image_height = (image_width as f64 / aspect_ratio) as i32;
  let samples_per_pixel = 200;
  let max_depth = 50;
  let background = Color::zero();//Color::new(0.70, 0.80, 1.00);

  // World

  let world = cornell_box();

  // Camera
  let lookfrom = Point3::new(278.0, 278.0, -800.0);
  let lookat = Point3::new(278.0, 278.0, 0.0);
  let vup = Vec3::new(0.0,1.0,0.0);
  let dist_to_focus = 10.0;
  let aperture = 0.0;

  let cam = Camera::new(lookfrom, lookat, vup, 40.0, aspect_ratio, aperture, dist_to_focus, 0.0, 1.0);

  //Render

  let cpus = num_cpus::get();

  let bar = ProgressBar::new(image_width as u64 * image_height as u64 * cpus as u64);

  eprintln!("Workload distribution: {:?}", divide_into_parts(samples_per_pixel, cpus as i32));

  let result = divide_into_parts(samples_per_pixel, cpus as i32)
    .into_par_iter()
    .map(|samples| {
      render_image(&cam, &world, &background, image_width, image_height, samples, max_depth, &bar)
    })
    .reduce(
      || { vec![Color::new(0.0, 0.0, 0.0); (image_width * image_height) as usize] },
      |a, b| a.into_iter().zip(b.into_iter()).map(|(c1, c2)| c1 + c2).collect()
    );

  println!("P3\n{image_width} {image_height}\n255");
  for pixel_color in result {
    write_color(pixel_color / samples_per_pixel as f64);
  }

  bar.finish();
}