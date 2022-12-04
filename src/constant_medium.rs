use std::f64::{NEG_INFINITY, INFINITY};

use crate::{hittable::{Hittable, HitRecord}, material::{Isotropic}, texture::{Texture, SolidColor}, vec3::{Color, Vec3}, aabb::AABB, util::random_double};

pub struct ConstantMedium<H: Hittable, T: Texture> {
  boundary: H,
  phase_function: Isotropic<T>,
  neg_inv_density: f64
}

impl<H: Hittable, T: Texture> ConstantMedium<H, T> {
  pub fn new(b: H, d: f64, a: T) -> Self {
    Self { boundary: b, neg_inv_density: -1.0/d, phase_function: Isotropic::new(a) }
  }
}

impl<H: Hittable> ConstantMedium<H, SolidColor> {
  pub fn solid(b: H, d: f64, c: Color) -> Self {
    Self { boundary: b, neg_inv_density: -1.0/d, phase_function: Isotropic::solid(c) }
  }
}

impl<H: Hittable, T: Texture> Hittable for ConstantMedium<H, T> {
  fn hit(&self, r: &crate::ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
    // Print occasional samples when debugging. To enable, set enableDebug true.
    let enable_debug = false;
    let debugging = enable_debug && random_double() < 0.00001;

    let mut rec1 = match self.boundary.hit(r, NEG_INFINITY, INFINITY) {
      Some(rec) => rec,
      None => return None
    };

    let mut rec2 = match self.boundary.hit(r, rec1.t+0.0001, INFINITY) {
      Some(rec) => rec,
      None => return None
    };

    if debugging { eprintln!("\nt_min={}, tmax={}", rec1.t, rec2.t) };

    if rec1.t < t_min { rec1.t = t_min };
    if rec2.t > t_max { rec2.t = t_max };

    if rec1.t >= rec2.t { return None }

    if rec1.t < 0.0 { rec1.t = 0.0 }

    let ray_length = r.direction().length();
    let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
    let hit_distance = self.neg_inv_density * random_double().ln();

    if hit_distance > distance_inside_boundary { return None }

    let t = rec1.t + hit_distance / ray_length;
    let p = r.at(t);

    if debugging {
      eprintln!("hit_distance = {hit_distance}\nrec.t = {t}\nrec.p = {p}");
    }

    let normal = Vec3::new(1.0, 0.0, 0.0); // arbitrary
    let front_face = true; // also arbitrary

    Some(HitRecord { p, normal, material: &self.phase_function, t, u: 0.0, v: 0.0, front_face })
  }

  fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
    self.boundary.bounding_box(time0, time1)
  }
}