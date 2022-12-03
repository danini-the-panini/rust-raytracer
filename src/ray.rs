use crate::vec3::{Vec3, Point3};

#[derive(Debug, Clone, Copy)]
pub struct Ray {
  dir: Vec3,
  orig: Point3,
  tm: f64
}

impl Ray {
  pub fn new(origin: Point3, direction: Vec3, time: f64) -> Self {
    Ray { orig: origin, dir: direction, tm: time }
  }

  pub fn direction(&self) -> Point3 { self.dir }
  pub fn origin(&self) -> Point3 { self.orig }
  pub fn time(&self) -> f64 { self.tm }

  pub fn at(&self, t: f64) -> Point3 {
    self.orig + t*self.dir
  }
}