use crate::{vec3::{Point3, dot, Vec3}, hittable::{Hittable, HitRecord}, ray::Ray, material::Material, aabb::AABB};

pub struct Sphere<M: Material> {
  pub center: Point3,
  pub radius: f64,
  pub material: M,
}

impl<M: Material> Hittable for Sphere<M> {
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

    let t = root;
    let p = r.at(t);
    let outward_normal = (p - self.center) / self.radius;
    let mut rec = HitRecord { t, p, material: &self.material, normal: outward_normal, front_face: true };
    rec.set_face_normal(r, &outward_normal);

    Some(rec)
  }

  fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
    let radius_vec = Vec3::new(self.radius, self.radius, self.radius);
    Some(AABB::new(
      self.center - radius_vec,
      self.center + radius_vec
    ))
  }
}