use crate::vec3::{Vec3, Point3};

#[derive(Debug, Clone, Copy)]
pub struct Ray {
  dir: Vec3,
  orig: Point3
}

impl Ray {
  pub fn new(origin: Point3, direction: Vec3) -> Self {
    Ray { orig: origin, dir: direction }
  }

  pub fn direction(&self) -> Point3 { self.dir }
  pub fn origin(&self) -> Point3 { self.orig }

  pub fn at(&self, t: f64) -> Point3 {
    self.orig + t*self.dir
  }
}