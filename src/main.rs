mod vec3;
mod color;
mod ray;

use crate::vec3::{Vec3, Point3, Color, unit_vector, dot};
use crate::color::write_color;
use crate::ray::Ray;

fn hit_sphere(center: &Point3, radius: f64, r: &Ray) -> bool {
  let oc = r.origin() - *center;
  let a = dot(&r.direction(), &r.direction());
  let b = 2.0 * dot(&oc, &r.direction());
  let c = dot(&oc, &oc) - radius*radius;
  let discriminant = b*b - 4.0*a*c;
  discriminant > 0.0
}

fn ray_color(r: &Ray) -> Color {
  if hit_sphere(&Point3::new(0.0, 0.0, -1.0), 0.5, r) {
    return Color::new(1.0, 0.0, 0.0);
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

  // Camera

  let viewport_height = 2.0;
  let viewport_width = aspect_ratio * viewport_height;
  let focal_length = 1.0;

  let origin = Point3::new(0.0, 0.0, 0.0);
  let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
  let vertical = Vec3::new(0.0, viewport_height, 0.0);
  let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - Vec3::new(0.0, 0.0, focal_length);

  //Render

  println!("P3\n{image_width} {image_height}\n255");

  for j in (0..image_height).rev() {
    eprint!("\rScanlines remaining: {j} ");
    for i in 0..image_width {
      let u = i as f64 / (image_width as f64 - 1.0);
      let v = j as f64 / (image_height as f64 - 1.0);
      let r = Ray::new(origin, lower_left_corner + u*horizontal + v*vertical - origin);
      let pixel_color = ray_color(&r);
      write_color(pixel_color);
    }
  }
  eprintln!("\nDone")
}