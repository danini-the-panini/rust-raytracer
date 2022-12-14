use crate::{vec3::Point3, hittable_list::HittableList, material::Material, aarect::{XYRect, XZRect, YZRect}, hittable::{Hittable, HitRecord}, aabb::AABB};

pub struct Cube {
  box_min: Point3,
  box_max: Point3,
  sides: HittableList
}

impl Cube {
  pub fn new<M: Material + Copy + 'static>(p0: Point3, p1: Point3, material: M) -> Self {
    let mut sides = HittableList::new();

    sides.add(Box::new(XYRect::new(p0.x(), p1.x(), p0.y(), p1.y(), p1.z(), material)));
    sides.add(Box::new(XYRect::new(p0.x(), p1.x(), p0.y(), p1.y(), p0.z(), material)));

    sides.add(Box::new(XZRect::new(p0.x(), p1.x(), p0.z(), p1.z(), p1.y(), material)));
    sides.add(Box::new(XZRect::new(p0.x(), p1.x(), p0.z(), p1.z(), p0.y(), material)));

    sides.add(Box::new(YZRect::new(p0.y(), p1.y(), p0.z(), p1.z(), p1.x(), material)));
    sides.add(Box::new(YZRect::new(p0.y(), p1.y(), p0.z(), p1.z(), p0.x(), material)));

    Self { box_min: p0, box_max: p1, sides }
  }
}

impl Hittable for Cube {
  fn hit(&self, r: &crate::ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
    self.sides.hit(r, t_min, t_max)
  }

  fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
    Some(AABB::new(self.box_min, self.box_max))
  }
}