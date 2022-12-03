use crate::{ray::Ray, hittable::HitRecord, vec3::{Color, Vec3, reflect, unit_vector, dot}};

pub trait Material {
  fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
}

#[derive(Debug, Clone, Copy)]
pub struct Lambertian {
  pub albedo: Color
}

impl Material for Lambertian {
  fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
      let mut scatter_direction = rec.normal + Vec3::random_unit_vector();

      // Catch degenerate scatter direction
      if scatter_direction.near_zero() {
        scatter_direction = rec.normal;
      }

      Some((self.albedo, Ray::new(rec.p, scatter_direction)))
  }
}

#[derive(Debug, Clone, Copy)]
pub struct Metal {
  pub albedo: Color
}

impl Material for Metal {
  fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
      let reflected = reflect(&unit_vector(r_in.direction()), &rec.normal);
      let scattered = Ray::new(rec.p, reflected);

      if dot(&scattered.direction(), &rec.normal) > 0.0 {
        Some((self.albedo, scattered))
      } else {
        None
      }
  }
}