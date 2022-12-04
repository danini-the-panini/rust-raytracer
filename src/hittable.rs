use std::f64::{INFINITY, NEG_INFINITY};

use crate::{vec3::{Point3, Vec3, dot}, ray::Ray, material::Material, aabb::AABB, util::{fmin, fmax}};

pub struct HitRecord<'a> {
  pub p: Point3,
  pub normal: Vec3,
  pub material: &'a dyn Material,
  pub t: f64,
  pub u: f64,
  pub v: f64,
  pub front_face: bool
}

impl HitRecord<'_> {
  pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
    self.front_face = dot(&r.direction(), outward_normal) < 0.0;
    self.normal = if self.front_face { *outward_normal } else { -*outward_normal };
  }
}

pub trait Hittable: Sync {
  fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
  fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB>;
}

pub struct Translate<H: Hittable> {
  hittable: H,
  offset: Vec3
}

impl<H: Hittable> Translate<H> {
  pub fn new(hittable: H, offset: Vec3) -> Self { Self { hittable, offset } }
}

impl<H: Hittable> Hittable for Translate<H> {
  fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
    let moved_r = Ray::new(r.origin() - self.offset, r.direction(), r.time());
    if let Some(mut rec) = self.hittable.hit(&moved_r, t_min, t_max) {
      rec.p += self.offset;
      let normal = rec.normal;
      rec.set_face_normal(&moved_r, &normal);

      Some(rec)
    } else {
      None
    }
  }

  fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
    if let Some(b) = self.hittable.bounding_box(time0, time1) {
      Some(AABB::new(b.min() + self.offset, b.max() + self.offset))
    } else {
      None
    }
  }
}

pub struct RotateY<H: Hittable> {
  hittable: H,
  sin_theta: f64,
  cos_theta: f64,
  bbox: Option<AABB>
}

impl<H: Hittable> RotateY<H> {
  pub fn new(hittable: H, angle: f64) -> Self {
    let radians = angle.to_radians();
    let sin_theta = (radians).sin();
    let cos_theta = (radians).cos();
    if let Some(bbox) = hittable.bounding_box(0.0, 1.0) {

      let mut min = Point3::new(INFINITY, INFINITY, INFINITY);
      let mut max = Point3::new(NEG_INFINITY, NEG_INFINITY, NEG_INFINITY);

      for i in 0..2 {
        for j in 0..2 {
          for k in 0..2 {
            let x = (i as f64)*bbox.max().x() + (1.0-i as f64)*bbox.min().x();
            let y = (j as f64)*bbox.max().y() + (1.0-j as f64)*bbox.min().y();
            let z = (k as f64)*bbox.max().z() + (1.0-k as f64)*bbox.min().z();

            let x = cos_theta*x + sin_theta*z;
            let z = -sin_theta*x + cos_theta*z;

            let tester = Vec3::new(x, y, z);

            for c in 0..3 {
              min[c] = fmin(min[c], tester[c]);
              max[c] = fmax(max[c], tester[c]);
            }
          }
        }
      }

      Self { hittable, sin_theta, cos_theta, bbox: Some(AABB::new(min, max)) }
    } else {
      Self { hittable, sin_theta, cos_theta, bbox: None }
    }
  }
}

impl<H: Hittable> Hittable for RotateY<H> {
  fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
    let mut origin = r.origin();
    let mut direction = r.direction();

    origin[0] = self.cos_theta*r.origin()[0] - self.sin_theta*r.origin()[2];
    origin[2] = self.sin_theta*r.origin()[0] + self.cos_theta*r.origin()[2];

    direction[0] = self.cos_theta*r.direction()[0] - self.sin_theta*r.direction()[2];
    direction[2] = self.sin_theta*r.direction()[0] + self.cos_theta*r.direction()[2];

    let rotated_r = Ray::new(origin, direction, r.time());

    if let Some(mut rec) = self.hittable.hit(&rotated_r, t_min, t_max) {
      let mut p = rec.p;
      let mut normal = rec.normal;

      p[0] = self.cos_theta*rec.p[0] + self.sin_theta*rec.p[2];
      p[2] = -self.sin_theta*rec.p[0] + self.cos_theta*rec.p[2];

      normal[0] = self.cos_theta*rec.normal[0] + self.sin_theta*rec.normal[2];
      normal[2] = self.sin_theta*rec.normal[0] + self.cos_theta*rec.normal[2];

      rec.p = p;
      rec.set_face_normal(&rotated_r, &normal);

      Some(rec)
    } else {
      None
    }
  }

  fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
    self.bbox
  }
}