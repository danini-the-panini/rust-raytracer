use crate::{hittable::{Hittable, HitRecord}, ray::Ray};

pub struct HittableList {
  objects: Vec<Box<dyn Hittable>>
}

impl HittableList {
  pub fn new() -> Self { HittableList { objects: Vec::new() } }
  pub fn add(&mut self, object: Box<dyn Hittable>) { self.objects.push(object) }
}

impl Hittable for HittableList {
  fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
    let mut rec : Option<HitRecord> = None;
    let mut closest_so_far = t_max;

    for object in &self.objects {
      if let Some(temp_rec) = object.hit(r, t_min, closest_so_far) {
        closest_so_far = temp_rec.t;
        rec = Some(temp_rec);
      }
    }

    rec
  }
}