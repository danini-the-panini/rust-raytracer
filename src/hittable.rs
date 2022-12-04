use crate::{vec3::{Point3, Vec3, dot}, ray::Ray, material::Material, aabb::AABB};

pub struct HitRecord<'a> {
  pub p: Point3,
  pub normal: Vec3,
  pub material: &'a dyn Material,
  pub t: f64,
  pub front_face: bool,
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