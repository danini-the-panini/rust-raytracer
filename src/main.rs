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

use crate::camera::Camera;
use crate::hittable::Hittable;
use crate::hittable_list::HittableList;
use crate::material::{Lambertian, Metal};
use crate::sphere::Sphere;
use crate::util::random_double;
use crate::vec3::{Point3, Color, unit_vector};
use crate::color::write_color;
use crate::ray::Ray;

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

fn main() {
  // Image

  let aspect_ratio = 16.0 / 9.0;
  let image_width = 400;
  let image_height = (image_width as f64 / aspect_ratio) as i32;
  let samples_per_pixel = 100;
  let max_depth = 50;

  // World

  let mut world = HittableList::new();

  let material_ground = Lambertian { albedo: Color::new(0.8, 0.8, 0.0) };
  let material_center = Lambertian { albedo: Color::new(0.7, 0.3, 0.3) };
  let material_left = Metal { albedo: Color::new(0.8, 0.8, 0.8), fuzz: 0.3 };
  let material_right = Metal { albedo: Color::new(0.8, 0.6, 0.2), fuzz: 1.0 };

  world.add(Box::new(Sphere { center: Point3::new(0.0,-100.5,-1.0), radius: 100.0, material: Rc::new(material_ground) }));
  world.add(Box::new(Sphere { center: Point3::new( 0.0, 0.0, -1.0), radius: 0.5, material: Rc::new(material_center) }));
  world.add(Box::new(Sphere { center: Point3::new(-1.0, 0.0, -1.0), radius: 0.5, material: Rc::new(material_left) }));
  world.add(Box::new(Sphere { center: Point3::new( 1.0, 0.0, -1.0), radius: 0.5, material: Rc::new(material_right) }));


  // Camera
  let cam = Camera::new();

  //Render

  println!("P3\n{image_width} {image_height}\n255");

  for j in (0..image_height).rev() {
    eprint!("\rScanlines remaining: {j} ");
    for i in 0..image_width {
      let mut pixel_color = Color::new(0.0, 0.0, 0.0);
      for _s in 0..samples_per_pixel {
        let u = (i as f64 + random_double()) / (image_width as f64 - 1.0);
        let v = (j as f64 + random_double()) / (image_height as f64 - 1.0);
        let r = cam.get_ray(u, v);
        pixel_color += ray_color(&r, &world, max_depth);
      }
      write_color(pixel_color, samples_per_pixel);
    }
  }
  eprintln!("\nDone")
}