use std::f64::consts::PI;

use crate::{vec3::{Point3, dot, Vec3}, hittable::{Hittable, HitRecord}, ray::Ray, material::Material, aabb::AABB};

#[derive(Debug, Clone, Copy)]
pub struct Sphere<M: Material> {
  center: Point3,
  radius: f64,
  material: M,
}

pub fn get_sphere_uv(p: &Point3) -> (f64, f64) {
  // p: a given point on the sphere of radius one, centered at the origin.
  // u: returned value [0,1] of angle around the Y axis from X=-1.
  // v: returned value [0,1] of angle from Y=-1 to Y=+1.
  //     <1 0 0> yields <0.50 0.50>       <-1  0  0> yields <0.00 0.50>
  //     <0 1 0> yields <0.50 1.00>       < 0 -1  0> yields <0.50 0.00>
  //     <0 0 1> yields <0.25 0.50>       < 0  0 -1> yields <0.75 0.50>

  let theta = f64::acos(-p.y());
  let phi = f64::atan2(-p.z(), p.x()) + PI;

  (phi / (2.0*PI), theta / PI)
}

impl<M: Material> Sphere<M> {
  pub fn new(center: Point3, radius: f64, material: M) -> Self { Self { center, radius, material } }
}

impl<M: Material> Hittable for Sphere<M> {
  fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
    let oc = r.origin() - self.center;
    let a = r.direction().length_squared();
    let half_b = dot(&oc, &r.direction());
    let c = oc.length_squared() - self.radius*self.radius;

    let discriminant = half_b*half_b - a*c;
    if discriminant < 0.0 { return None }
    let sqrtd = discriminant.sqrt();

    // Find the nearest root that lies in the acceptable range.
    let mut root = (-half_b - sqrtd) / a;
    if root < t_min || t_max < root {
      root = (-half_b + sqrtd) / a;
      if root < t_min || t_max < root { return None }
    }

    let t = root;
    let p = r.at(t);
    let outward_normal = (p - self.center) / self.radius;
    let (u, v) = get_sphere_uv(&outward_normal);
    let mut rec = HitRecord { t, p, material: &self.material, normal: outward_normal, front_face: true, u, v };
    rec.set_face_normal(r, &outward_normal);

    Some(rec)
  }

  fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
    let radius_vec = Vec3::new(self.radius, self.radius, self.radius);
    Some(AABB::new(
      self.center - radius_vec,
      self.center + radius_vec
    ))
  }
}