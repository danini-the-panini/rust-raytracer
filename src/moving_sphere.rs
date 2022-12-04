use crate::{vec3::{Point3, dot, Vec3}, hittable::{Hittable, HitRecord}, ray::Ray, material::Material, aabb::AABB, sphere::get_sphere_uv};

pub struct MovingSphere<M: Material> {
  center0: Point3, pub center1: Point3,
  time0: f64, pub time1: f64,
  radius: f64,
  material: M,
}

impl<M: Material> MovingSphere<M> {
  pub fn new(center0: Point3, center1: Point3, time0: f64, time1: f64, radius: f64, material: M) -> Self {
    Self { center0, center1, time0, time1, radius, material }
  }

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
    let (u, v) = get_sphere_uv(&outward_normal);
    let mut rec = HitRecord { t, p, material: &self.material, normal: outward_normal, front_face: true, u, v };
    rec.set_face_normal(r, &outward_normal);

    Some(rec)
  }

  fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
    let radius_vec = Vec3::new(self.radius, self.radius, self.radius);
    let box0 = AABB::new(
      self.center(time0) - radius_vec,
      self.center(time0) + radius_vec
    );
    let box1 = AABB::new(
      self.center(time1) - radius_vec,
      self.center(time1) + radius_vec
    );
    Some(AABB::surrounding_box(&box0, &box1))
  }
}