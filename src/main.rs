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

use std::f64::INFINITY;
use bvh::BVH;
use indicatif::ProgressBar;
use moving_sphere::MovingSphere;
use rayon::prelude::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use texture::{CheckerTexture, NoiseTexture};

use crate::camera::Camera;
use crate::hittable::Hittable;
use crate::material::{Lambertian, Metal, Dialectric};
use crate::sphere::Sphere;
use crate::util::{random_double, random_double_in_range, divide_into_parts};
use crate::vec3::{Point3, Color, unit_vector, Vec3};
use crate::color::write_color;
use crate::ray::Ray;

fn render_image(cam: &Camera, world: &dyn Hittable, image_width: i32, image_height: i32, samples_per_pixel: i32, max_depth: i32, bar: &ProgressBar) -> Vec<Color> {
  let mut output : Vec<Color> = Vec::new();
  for j in (0..image_height).rev() {
    for i in 0..image_width {
      let mut pixel_color = Color::new(0.0, 0.0, 0.0);
      for _ in 0..samples_per_pixel {
        let u = (i as f64 + random_double()) / (image_width as f64 - 1.0);
        let v = (j as f64 + random_double()) / (image_height as f64 - 1.0);
        let r = cam.get_ray(u, v);
        pixel_color += ray_color(&r, world, max_depth);
      }
      output.push(pixel_color);
      bar.inc(1);
    }
  }
  output
}

fn ray_color(r: &Ray, world: &dyn Hittable, depth: i32) -> Color {
  // If we've exceeded the ray bounce limit, no more light is gathered.
  if depth <= 0 { return Color::new(0.0, 0.0, 0.0) };

  if let Some(rec) = world.hit(r, 0.001, INFINITY) {
    if let Some((attenutation, scattered)) = rec.material.scatter(r, &rec) {
      return attenutation * ray_color(&scattered, world, depth - 1);
    }
    return Color::new(0.0, 0.0, 0.0);
  }

  let unit_direction = unit_vector(r.direction());
  let t = 0.5*(unit_direction.y() + 1.0);
  (1.0-t)*Color::new(1.0, 1.0, 1.0) + t*Color::new(0.5, 0.7, 1.0)
}

fn random_scene() -> BVH {
  let mut world: Vec<Box<dyn Hittable>> = Vec::new();

  let checker = CheckerTexture::solid(Color::new(0.2,0.3,0.1), Color::new(0.9, 0.9, 0.9));
  world.push(Box::new(Sphere { center: Point3::new(0.0,-1000.0,0.0), radius: 1000.0, material: Lambertian::new(checker) }));

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
          world.push(Box::new(MovingSphere { center0: center, center1, time0: 0.0, time1: 1.0, radius: 0.2, material }));
        } else if choose_mat < 0.95 {
          // metal
          let albedo = Color::random_in_range(0.5, 1.0);
          let fuzz = random_double_in_range(0.0, 0.5);
          let material = Metal::solid(albedo, fuzz);
          world.push(Box::new(Sphere { center, radius: 0.2, material }));
        } else {
          // glass
          let material = Dialectric { ir: 1.5 };
          world.push(Box::new(Sphere { center, radius: 0.2, material }));
        }
      }
    }
  }

  let material1 = Dialectric { ir: 1.5 };
  world.push(Box::new(Sphere { center: Point3::new(0.0, 1.0, 0.0), radius: 1.0, material: material1 }));

  let material2 = Lambertian::solid(Color::new(0.4, 0.2, 0.1));
  world.push(Box::new(Sphere { center: Point3::new(-4.0, 1.0, 0.0), radius: 1.0, material: material2 }));

  let material3 = Metal::solid(Color::new(0.7, 0.6, 0.5), 0.0);
  world.push(Box::new(Sphere { center: Point3::new(4.0, 1.0, 0.0), radius: 1.0, material: material3 }));

  BVH::new(world, 0.0, 1.0)
}

fn two_spheres() -> BVH {
  let mut world: Vec<Box<dyn Hittable>> = Vec::new();

  let checker = CheckerTexture::solid(Color::new(0.2,0.3,0.1), Color::new(0.9, 0.9, 0.9));

  world.push(Box::new(Sphere { center: Point3::new(0.0,-10.0,0.0), radius: 10.0, material: Lambertian::new(checker) }));
  world.push(Box::new(Sphere { center: Point3::new(0.0,10.0,0.0), radius: 10.0, material: Lambertian::new(checker) }));

  BVH::new(world, 0.0, 0.0)
}

fn two_perlin_spheres() -> BVH {
  let mut world: Vec<Box<dyn Hittable>> = Vec::new();

  let pertext = NoiseTexture::new(4.0);

  world.push(Box::new(Sphere { center: Point3::new(0.0,-1000.0,0.0), radius: 1000.0, material: Lambertian::new(pertext) }));
  world.push(Box::new(Sphere { center: Point3::new(0.0,2.0,0.0), radius: 2.0, material: Lambertian::new(pertext) }));

  BVH::new(world, 0.0, 0.0)
}

fn main() {
  // Image

  let aspect_ratio = 16.0 / 9.0;
  let image_width = 400;
  let image_height = (image_width as f64 / aspect_ratio) as i32;
  let samples_per_pixel = 50;
  let max_depth = 50;

  // World

  let world = two_perlin_spheres();

  // Camera
  let lookfrom = Point3::new(12.0, 2.0, 3.0);
  let lookat = Point3::new(0.0,0.0,0.0);
  let vup = Vec3::new(0.0,1.0,0.0);
  let dist_to_focus = 10.0;
  let aperture = 0.1;

  let cam = Camera::new(lookfrom, lookat, vup, 20.0, aspect_ratio, aperture, dist_to_focus, 0.0, 1.0);

  //Render

  let cpus = num_cpus::get();

  let bar = ProgressBar::new(image_width as u64 * image_height as u64 * cpus as u64);

  eprintln!("Workload distribution: {:?}", divide_into_parts(samples_per_pixel, cpus as i32));

  let result = divide_into_parts(samples_per_pixel, cpus as i32)
    .into_par_iter()
    .map(|samples| {
      render_image(&cam, &world, image_width, image_height, samples, max_depth, &bar)
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