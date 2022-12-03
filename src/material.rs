use crate::{ray::Ray, hittable::HitRecord, vec3::{Color, Vec3, reflect, unit_vector, dot, refract}};

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
  pub albedo: Color,
  pub fuzz: f64
}

impl Material for Metal {
  fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
      let reflected = reflect(&unit_vector(r_in.direction()), &rec.normal);
      let scattered = Ray::new(rec.p, reflected + self.fuzz*Vec3::random_in_unit_sphere());

      if dot(&scattered.direction(), &rec.normal) > 0.0 {
        Some((self.albedo, scattered))
      } else {
        None
      }
  }
}

#[derive(Debug, Clone, Copy)]
pub struct Dialectric {
  pub ir: f64
}

impl Material for Dialectric {
  fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
    let refraction_ratio = if rec.front_face { 1.0/self.ir } else { self.ir };

    let unit_direction = unit_vector(r_in.direction());
    let a = dot(&-unit_direction, &rec.normal);
    let cos_theta = if a < 1.0 { a } else { 1.0 };
    let sin_theta = (1.0 - cos_theta*cos_theta).sqrt();

    let cannot_refract = refraction_ratio * sin_theta > 1.0;

    let direction = if cannot_refract {
      reflect(&unit_direction, &rec.normal)
    } else {
      refract(&unit_direction, &rec.normal, refraction_ratio)
    };

    Some((
      Color::new(1.0, 1.0, 1.0),
      Ray::new(rec.p, direction)
    ))
  }
}