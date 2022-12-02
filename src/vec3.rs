use std::{ops, fmt};

use crate::util::{random_double, random_double_in_range};

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
  e: [f64; 3]
}

pub type Point3 = Vec3;
pub type Color = Vec3;

impl Vec3 {
  pub fn zero() -> Self { Vec3 { e: [0.0, 0.0, 0.0] } }
  pub fn new(e0: f64, e1: f64, e2: f64) -> Self { Vec3 { e: [e0, e1, e2] } }

  pub fn random() -> Self {
    Vec3 { e: [random_double(), random_double(), random_double()] }
  }
  pub fn random_in_range(min: f64, max: f64) -> Self {
    Vec3 { e: [random_double_in_range(min, max), random_double_in_range(min, max), random_double_in_range(min, max)] }
  }
  pub fn random_in_unit_sphere() -> Self {
    loop {
      let p = Vec3::random_in_range(-1.0, 1.0);
      if p.length_squared() < 1.0 { break p };
    }
  }
  pub fn random_unit_vector() -> Self {
    unit_vector(Vec3::random_in_unit_sphere())
  }
  
  pub fn x(&self) -> f64 { self.e[0] }
  pub fn y(&self) -> f64 { self.e[1] }
  pub fn z(&self) -> f64 { self.e[2] }

  pub fn length(&self) -> f64 {
    self.length_squared().sqrt()
  }

  pub fn length_squared(&self) -> f64 {
    self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
  }
}

impl ops::Neg for Vec3 {
  type Output = Vec3;

  fn neg(self) -> Vec3 {
    Vec3::new(-self.e[0], -self.e[1], -self.e[2])
  }
}

impl ops::Index<usize> for Vec3 {
  type Output = f64;

  fn index(&self, i: usize) -> &f64 {
    &self.e[i]
  }
}

impl ops::AddAssign for Vec3 {
  fn add_assign(&mut self, v: Self) {
    self.e[0] += v.e[0];
    self.e[1] += v.e[1];
    self.e[2] += v.e[2];
  }
}

impl ops::MulAssign<f64> for Vec3 {
  fn mul_assign(&mut self, t: f64) {
    self.e[0] *= t;
    self.e[1] *= t;
    self.e[2] *= t;
  }
}

impl ops::DivAssign<f64> for Vec3 {
  fn div_assign(&mut self, t: f64) {
    *self *= 1.0/t;
  }
}

impl ops::Add for Vec3 {
  type Output = Vec3;

  fn add(self, v: Self) -> Vec3 {
    Vec3::new(self.e[0] + v.e[0], self.e[1] + v.e[1], self.e[2] + v.e[2])
  }
}

impl ops::Sub for Vec3 {
  type Output = Vec3;

  fn sub(self, v: Self) -> Vec3 {
    Vec3::new(self.e[0] - v.e[0], self.e[1] - v.e[1], self.e[2] - v.e[2])
  }
}

impl ops::Mul<f64> for Vec3 {
  type Output = Vec3;

  fn mul(self, t: f64) -> Vec3 {
    Vec3::new(self.e[0] * t, self.e[1] * t, self.e[2] * t)
  }
}

impl ops::Mul for Vec3 {
  type Output = Vec3;

  fn mul(self, v: Self) -> Vec3 {
    Vec3::new(self.e[0] * v.e[0], self.e[1] * v.e[1], self.e[2] * v.e[2])
  }
}

impl ops::Mul<Vec3> for f64 {
  type Output = Vec3;

  fn mul(self, v: Vec3) -> Vec3 {
    Vec3::new(self * v.e[0], self * v.e[1], self * v.e[2])
  }
}

impl ops::Div<f64> for Vec3 {
  type Output = Vec3;

  fn div(self, t: f64) -> Vec3 {
    (1.0 / t) * self
  }
}

impl fmt::Display for Vec3 {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{} {} {}", self.e[0], self.e[1], self.e[2])
  }
}

pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
  u.e[0] * v.e[0] +
  u.e[1] * v.e[1] +
  u.e[2] * v.e[2]
}

pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
  Vec3::new(
    u.e[1] * v.e[2] - u.e[2] * v.e[1],
    u.e[2] * v.e[0] - u.e[0] * v.e[2],
    u.e[0] * v.e[1] - u.e[1] * v.e[0]
  )
}

pub fn unit_vector(v: Vec3) -> Vec3 {
  v / v.length()
}
