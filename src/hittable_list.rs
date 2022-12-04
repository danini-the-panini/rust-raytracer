use crate::{hittable::{Hittable, HitRecord}, aabb::AABB};

pub struct HittableList {
  objects: Vec<Box<dyn Hittable>>
}

impl HittableList {
  pub fn new() -> Self { Self { objects: Vec::new() } }
  pub fn add(&mut self, object: Box<dyn Hittable>) {
    self.objects.push(object);
  }
}

impl Hittable for HittableList {
  fn hit(&self, r: &crate::ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
    let mut hit_record: Option<HitRecord> = None;
    let mut closest_so_far = t_max;

    for object in &self.objects {
      if let Some(temp_rec) = object.hit(r, t_min, closest_so_far) {
        closest_so_far = temp_rec.t;
        hit_record = Some(temp_rec);
      }
    }

    hit_record
  }

  fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
    if self.objects.is_empty() { return None }

    let mut output_box: Option<AABB> = None;

    for object in &self.objects {
      match object.bounding_box(time0, time1) {
        Some(temp_box) => {
          output_box = Some(if let Some(b) = output_box { AABB::surrounding_box(&b, &temp_box) } else { temp_box })
        },
        None => { return None }
      }
    }

    output_box
  }
}