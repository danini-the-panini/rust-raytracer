use crate::{material::Material, hittable::{Hittable, HitRecord}, aabb::AABB, vec3::{Point3, Vec3}};

#[derive(Debug, Clone, Copy)]
pub struct XYRect<M: Material> {
  material: M,
  x0: f64, x1: f64, y0: f64, y1: f64, k: f64
}

impl<M: Material> XYRect<M> {
  pub fn new(x0: f64, x1: f64, y0: f64, y1: f64, k: f64, material: M) -> Self {
    Self { x0, x1, y0, y1, k, material }
  }
}

impl<M: Material> Hittable for XYRect<M> {
  fn hit(&self, r: &crate::ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
    let t = (self.k-r.origin().z()) / r.direction().z();
    if t < t_min || t > t_max { return None };

    let x = r.origin().x() + t*r.direction().x();
    let y = r.origin().y() + t*r.direction().y();
    if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 { return None };

    let mut rec = HitRecord {
      u: (x-self.x0)/(self.x1-self.x0),
      v: (y-self.y0)/(self.y1-self.y0),
      t,
      normal: Vec3::zero(),
      front_face: true,
      p: r.at(t),
      material: &self.material
    };
    rec.set_face_normal(r, &Vec3::new(0.0, 0.0, 1.0));
    Some(rec)
  }

  fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<crate::aabb::AABB> {
    Some(AABB::new(Point3::new(self.x0, self.y0, self.k-0.0001), Point3::new(self.x1, self.y1, self.k+0.0001)))
  }
}

#[derive(Debug, Clone, Copy)]
pub struct XZRect<M: Material> {
  material: M,
  x0: f64, x1: f64, z0: f64, z1: f64, k: f64
}

impl<M: Material> XZRect<M> {
  pub fn new(x0: f64, x1: f64, z0: f64, z1: f64, k: f64, material: M) -> Self {
    Self { x0, x1, z0, z1, k, material }
  }
}

impl<M: Material> Hittable for XZRect<M> {
  fn hit(&self, r: &crate::ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
    let t = (self.k-r.origin().y()) / r.direction().y();
    if t < t_min || t > t_max { return None };

    let x = r.origin().x() + t*r.direction().x();
    let z = r.origin().z() + t*r.direction().z();
    if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 { return None };

    let mut rec = HitRecord {
      u: (x-self.x0)/(self.x1-self.x0),
      v: (z-self.z0)/(self.z1-self.z0),
      t,
      normal: Vec3::zero(),
      front_face: true,
      p: r.at(t),
      material: &self.material
    };
    rec.set_face_normal(r, &Vec3::new(0.0, 1.0, 0.0));
    Some(rec)
  }

  fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<crate::aabb::AABB> {
    Some(AABB::new(Point3::new(self.x0, self.k-0.0001, self.z0), Point3::new(self.x1, self.k+0.0001, self.z1)))
  }
}
#[derive(Debug, Clone, Copy)]
pub struct YZRect<M: Material> {
  material: M,
  y0: f64, y1: f64, z0: f64, z1: f64, k: f64
}

impl<M: Material> YZRect<M> {
  pub fn new(y0: f64, y1: f64, z0: f64, z1: f64, k: f64, material: M) -> Self {
    Self { y0, y1, z0, z1, k, material }
  }
}

impl<M: Material> Hittable for YZRect<M> {
  fn hit(&self, r: &crate::ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
    let t = (self.k-r.origin().x()) / r.direction().x();
    if t < t_min || t > t_max { return None };

    let y = r.origin().y() + t*r.direction().y();
    let z = r.origin().z() + t*r.direction().z();
    if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 { return None };

    let mut rec = HitRecord {
      u: (y-self.y0)/(self.y1-self.y0),
      v: (z-self.z0)/(self.z1-self.z0),
      t,
      normal: Vec3::zero(),
      front_face: true,
      p: r.at(t),
      material: &self.material
    };
    rec.set_face_normal(r, &Vec3::new(1.0, 0.0, 0.0));
    Some(rec)
  }

  fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<crate::aabb::AABB> {
    Some(AABB::new(Point3::new(self.k-0.0001, self.y0, self.z0), Point3::new(self.k+0.0001,self.y1,  self.z1)))
  }
}