use crate::vec3::{Point3, Color};

pub trait Texture: Sync {
  fn value(&self, u: f64, v: f64, p: &Point3) -> Color;
}

#[derive(Debug, Clone, Copy)]
pub struct SolidColor {
  color_value: Color
}

impl SolidColor {
  pub fn new(c: Color) -> Self { Self { color_value: c } }
  pub fn new_rgb(red: f64, green: f64, blue: f64) -> Self {
    Self { color_value: Color::new(red, green, blue) }
  }
}

impl Texture for SolidColor {
  fn value(&self, _u: f64, _v: f64, _p: &Point3) -> Color {
    self.color_value
  }
}

#[derive(Debug, Clone, Copy)]
pub struct CheckerTexture<O: Texture, E: Texture> {
  odd: O,
  even: E
}

impl<O: Texture, E: Texture> CheckerTexture<O, E> {
  pub fn new(even: E, odd: O) -> Self { Self { odd, even } }
}

impl CheckerTexture<SolidColor, SolidColor> {
  pub fn solid(even: Color, odd: Color) -> Self {
    Self { odd: SolidColor::new(odd), even: SolidColor::new(even) }
  }
}

impl<O: Texture, E: Texture> Texture for CheckerTexture<O, E> {
  fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
      let sines = f64::sin(10.0*p.x())*f64::sin(10.0*p.y())*f64::sin(10.0*p.z());
      if sines < 0.0 {
        self.odd.value(u, v, p)
      } else {
        self.even.value(u, v, p)
      }
  }
}