mod util;
mod vec3;
mod color;
mod ray;
mod hittable;
mod sphere;
mod hittable_list;
mod camera;
mod material;

use std::f64::INFINITY;
use std::rc::Rc;
use std::sync::Arc;
use indicatif::ProgressBar;

use crate::camera::Camera;
use crate::hittable::Hittable;
use crate::hittable_list::HittableList;
use crate::material::{Lambertian, Metal, Dialectric};
use crate::sphere::Sphere;
use crate::util::{random_double, random_double_in_range};
use crate::vec3::{Point3, Color, unit_vector, Vec3};
use crate::color::write_color;
use crate::ray::Ray;

fn render_image(cam: Arc<Camera>, world: Arc<dyn Hittable>, image_width: i32, image_height: i32, samples_per_pixel: i32, max_depth: i32, bar: &ProgressBar) -> Vec<Color> {
  let mut output : Vec<Color> = Vec::new();
  for j in (0..image_height).rev() {
    for i in 0..image_width {
      let mut pixel_color = Color::new(0.0, 0.0, 0.0);
      for _s in 0..samples_per_pixel {
        let u = (i as f64 + random_double()) / (image_width as f64 - 1.0);
        let v = (j as f64 + random_double()) / (image_height as f64 - 1.0);
        let r = cam.get_ray(u, v);
        pixel_color += ray_color(&r, world.as_ref(), max_depth);
      }
      output.push(pixel_color / samples_per_pixel as f64);
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

fn random_scene() -> HittableList {
  let mut world = HittableList::new();

  let ground_material  = Lambertian { albedo: Color::new(0.5, 0.5, 0.5) };
  world.add(Box::new(Sphere { center: Point3::new(0.0,-1000.0,0.0), radius: 1000.0, material: Rc::new(ground_material) }));

  for a in -11..11 {
    for b in -11..11 {
      let choose_mat = random_double();
      let center = Point3::new(a as f64 + 0.9*random_double(), 0.2, b as f64 + 0.9*random_double());

      if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
        if choose_mat < 0.8 {
          // diffuse
          let albedo = Color::random() * Color::random();
          let material = Rc::new(Lambertian { albedo });
          world.add(Box::new(Sphere { center, radius: 0.2, material }));
        } else if choose_mat < 0.95 {
          // metal
          let albedo = Color::random_in_range(0.5, 1.0);
          let fuzz = random_double_in_range(0.0, 0.5);
          let material = Rc::new(Metal { albedo, fuzz });
          world.add(Box::new(Sphere { center, radius: 0.2, material }));
        } else {
          // glass
          let material = Rc::new(Dialectric { ir: 1.5 });
          world.add(Box::new(Sphere { center, radius: 0.2, material }));
        }
      }
    }
  }

  let material1 = Rc::new(Dialectric { ir: 1.5 });
  world.add(Box::new(Sphere { center: Point3::new(0.0, 1.0, 0.0), radius: 1.0, material: material1 }));

  let material2 = Rc::new(Lambertian { albedo: Color::new(0.4, 0.2, 0.1) });
  world.add(Box::new(Sphere { center: Point3::new(-4.0, 1.0, 0.0), radius: 1.0, material: material2 }));

  let material3 = Rc::new(Metal { albedo: Color::new(0.7, 0.6, 0.5), fuzz: 0.0 });
  world.add(Box::new(Sphere { center: Point3::new(4.0, 1.0, 0.0), radius: 1.0, material: material3 }));

  world
}

fn main() {
  // Image

  let aspect_ratio = 3.0 / 2.0;
  let image_width = 600;
  let image_height = (image_width as f64 / aspect_ratio) as i32;
  let samples_per_pixel = 10;
  let max_depth = 50;

  // World

  let world = Arc::new(random_scene());

  // Camera
  let lookfrom = Point3::new(12.0, 2.0, 3.0);
  let lookat = Point3::new(0.0,0.0,0.0);
  let vup = Vec3::new(0.0,1.0,0.0);
  let dist_to_focus = 10.0;
  let aperture = 0.1;
  
  let cam = Arc::new(Camera::new(lookfrom, lookat, vup, 20.0, aspect_ratio, aperture, dist_to_focus));

  //Render

  let bar = ProgressBar::new(image_width as u64 * image_height as u64);

  let output = render_image(cam, world, image_width, image_height, samples_per_pixel, max_depth, &bar);

  println!("P3\n{image_width} {image_height}\n255");
  for pixel_color in output {
    write_color(pixel_color);
  }

  bar.finish();
}