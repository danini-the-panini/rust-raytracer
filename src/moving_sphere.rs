use crate::{vec3::{Point3, dot}, hittable::{Hittable, HitRecord}, ray::Ray, material::Material};

pub struct MovingSphere<M: Material> {
  pub center0: Point3, pub center1: Point3,
  pub time0: f64, pub time1: f64,
  pub radius: f64,
  pub material: M,
}

impl<M: Material> MovingSphere<M> {
  fn center(&self, time: f64) -> Point3 {
    self.center0 + ((time - self.time0) / (self.time1 - self.time0))*(self.center1 - self.center0)
  }
}

impl<M: Material> Hittable for MovingSphere<M> {
  fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
    let center = self.center(r.time());
    let oc = r.origin() - center;
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

    let t = root;
    let p = r.at(t);
    let outward_normal = (p - center) / self.radius;
    let mut rec = HitRecord { t, p, material: &self.material, normal: outward_normal, front_face: true };
    rec.set_face_normal(r, &outward_normal);

    Some(rec)
  }
}