use crate::{vec3::Point3, ray::Ray, util::{max, min}};

#[derive(Debug, Clone, Copy)]
pub struct AABB {
  minimum: Point3,
  maximum: Point3
}

impl AABB {
  pub fn new(a: Point3, b: Point3) -> Self { Self { minimum: a, maximum: b } }

  pub fn surrounding_box(box0: &AABB, box1: &AABB) -> AABB {
    let small = Point3::new(
      min(box0.min().x(), box1.min().x()),
      min(box0.min().y(), box1.min().y()),
      min(box0.min().z(), box1.min().z()),
    );

    let big = Point3::new(
      max(box0.max().x(), box1.max().x()),
      max(box0.max().y(), box1.max().y()),
      max(box0.max().z(), box1.max().z()),
    );

    AABB::new(small, big)
  }

  pub fn min(&self) -> Point3 { self.minimum }
  pub fn max(&self) -> Point3 { self.maximum }

  pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> bool {
    for a in 0..3 {
      let inv_d = 1.0 / r.direction()[a];
      let mut t0 = (self.minimum[a] - r.origin()[a]) * inv_d;
      let mut t1 = (self.maximum[a] - r.origin()[a]) * inv_d;
      if inv_d < 0.0 { (t0, t1) = (t1, t0) };
      let t_min = max(t0, t_min);
      let t_max = min(t1, t_max);
      if t_max <= t_min { return false };
    }
    true
  }
}