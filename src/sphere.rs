use crate::{vec3::{Point3, dot}, hittable::{Hittable, HitRecord}, ray::Ray};

pub struct Sphere {
  pub center: Point3,
  pub radius: f64
}

impl Hittable for Sphere {
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

    let mut rec = HitRecord::new(root, &r.at(root));
    let outward_normal = (rec.p - self.center) / self.radius;
    rec.set_face_normal(r, &outward_normal);

    Some(rec)
  }
}