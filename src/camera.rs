use crate::{vec3::{Point3, Vec3, unit_vector, cross}, ray::Ray, util::random_double_in_range};

#[derive(Debug, Clone, Copy)]
pub struct Camera {
  origin: Point3,
  lower_left_corner: Point3,
  horizontal: Vec3,
  vertical: Vec3,
  u: Vec3, v: Vec3,
  lens_radius: f64,
  time0: f64,
  time1: f64
}

impl Camera {
  pub fn new(
    lookfrom: Point3,
    lookat: Point3,
    vup: Vec3,
    vfov: f64,
    aspect_ratio: f64,
    aperture: f64,
    focus_dist: f64,
    time0: f64,
    time1: f64
  ) -> Self {
    let theta = vfov.to_radians();
    let h = (theta/2.0).tan();
    let viewport_height = 2.0 * h;
    let viewport_width = aspect_ratio * viewport_height;

    let w = unit_vector(lookfrom - lookat);
    let u = unit_vector(cross(&vup, &w));
    let v = cross(&w, &u);

    let horizontal = focus_dist * viewport_width * u;
    let vertical = focus_dist * viewport_height * v;

    Camera {
      origin: lookfrom,
      horizontal,
      vertical,
      lower_left_corner: lookfrom - horizontal/2.0 - vertical/2.0 - focus_dist*w,
      u, v,
      lens_radius: aperture / 2.0,
      time0, time1
    }
  }

  pub fn get_ray(&self, s: f64, t: f64) -> Ray {
    let rd = self.lens_radius * Vec3::random_in_unit_disk();
    let offset = self.u * rd.x() + self.v * rd.y();

    Ray::new(
      self.origin + offset,
      self.lower_left_corner + s*self.horizontal + t*self.vertical - self.origin - offset,
      random_double_in_range(self.time0, self.time1)
    )
  }
}