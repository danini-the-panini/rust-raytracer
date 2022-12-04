use std::f64;
use crate::{ray::Ray, hittable::HitRecord, vec3::{Color, Vec3, reflect, unit_vector, dot, refract, Point3}, util::{random_double, min}, texture::{Texture, SolidColor}};

pub trait Material: Sync {
  fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
  fn emitted(&self, _u: f64, _v: f64, _p: &Point3) -> Color { Color::zero() }
}

#[derive(Debug, Clone, Copy)]
pub struct Lambertian<T: Texture> {
  pub albedo: T
}

impl<T: Texture> Lambertian<T> {
  pub fn new(a: T) -> Self { Self { albedo: a } }
}

impl Lambertian<SolidColor> {
  pub fn solid(a: Color) -> Self { Self { albedo: SolidColor::new(a) } }
}

impl<T: Texture> Material for Lambertian<T> {
  fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
    let mut scatter_direction = rec.normal + Vec3::random_unit_vector();

    // Catch degenerate scatter direction
    if scatter_direction.near_zero() {
      scatter_direction = rec.normal;
    }

    Some((
      self.albedo.value(rec.u, rec.v, &rec.p),
      Ray::new(rec.p, scatter_direction, r_in.time())
    ))
  }
}

#[derive(Debug, Clone, Copy)]
pub struct Metal<T: Texture> {
  pub albedo: T,
  pub fuzz: f64
}

impl<T: Texture> Metal<T> {
  pub fn new(a: T, fuzz: f64) -> Self { Self { albedo: a, fuzz } }
}

impl Metal<SolidColor> {
  pub fn solid(a: Color, fuzz: f64) -> Self { Self { albedo: SolidColor::new(a), fuzz } }
}

impl<T: Texture> Material for Metal<T> {
  fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
      let reflected = reflect(&unit_vector(r_in.direction()), &rec.normal);
      let scattered = Ray::new(rec.p, reflected + self.fuzz*Vec3::random_in_unit_sphere(), r_in.time());

      if dot(&scattered.direction(), &rec.normal) > 0.0 {
        Some((self.albedo.value(rec.u, rec.v, &rec.p), scattered))
      } else {
        None
      }
  }
}

#[derive(Debug, Clone, Copy)]
pub struct Dialectric {
  pub ir: f64
}

fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
  // Use Schlick's approximation for reflectance.
  let mut r0 = (1.0-ref_idx) / (1.0+ref_idx);
  r0 = r0*r0;
  r0 + (1.0-r0)*f64::powf(1.0 - cosine, 5.0)
}

impl Material for Dialectric {
  fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
    let refraction_ratio = if rec.front_face { 1.0/self.ir } else { self.ir };

    let unit_direction = unit_vector(r_in.direction());
    let cos_theta = min(dot(&-unit_direction, &rec.normal), 1.0);
    let sin_theta = (1.0 - cos_theta*cos_theta).sqrt();

    let cannot_refract = refraction_ratio * sin_theta > 1.0;

    let direction = if cannot_refract || reflectance(cos_theta, refraction_ratio) > random_double() {
      reflect(&unit_direction, &rec.normal)
    } else {
      refract(&unit_direction, &rec.normal, refraction_ratio)
    };

    Some((
      Color::new(1.0, 1.0, 1.0),
      Ray::new(rec.p, direction, r_in.time())
    ))
  }
}

#[derive(Debug, Clone, Copy)]
pub struct DiffuseLight<T: Texture> {
  emit: T
}

impl<T: Texture> DiffuseLight<T> {
  pub fn new(a: T) -> Self { Self { emit: a } }
}

impl DiffuseLight<SolidColor> {
  pub fn solid(c: Color) -> Self { Self { emit: SolidColor::new(c) } }
}

impl<T: Texture> Material for DiffuseLight<T> {
  fn scatter(&self, _r_in: &Ray, _rec: &HitRecord) -> Option<(Color, Ray)> {
    None
  }

  fn emitted(&self, u: f64, v: f64, p: &Point3) -> Color {
    self.emit.value(u, v, p)
  }
}
