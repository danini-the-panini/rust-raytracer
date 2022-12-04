use image::{io::Reader as ImageReader, RgbImage};

use crate::{vec3::{Point3, Color}, perlin::Perlin};

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

#[derive(Debug, Clone, Copy)]
pub struct NoiseTexture {
  noise: Perlin,
  scale: f64
}

impl NoiseTexture {
  pub fn new(scale: f64) -> Self { Self { noise: Perlin::new(), scale } }
}

impl Texture for NoiseTexture {
  fn value(&self, _u: f64, _v: f64, p: &Point3) -> Color {
    // Color::new(1.0,1.0,1.0) * 0.5 * (1.0 + self.noise.noise(&(self.scale * *p)))
    Color::new(1.0,1.0,1.0) * 0.5 * (1.0 + f64::sin(self.scale*p.z() + 10.0*self.noise.turb(p)))
  }
}

const BYTES_PER_PIXEL: i32 = 3;

#[derive(Debug, Clone)]
pub struct ImageTexture {
  img: Option<RgbImage>
}

impl ImageTexture {
  pub fn new(filename: &str) -> Self {
    let img = match ImageReader::open(filename) {
      Ok(reader) => {
        match reader.decode() {
          Ok(image) => image.as_rgb8().cloned(),
          Err(_) => None
        }
      },
      Err(_) => None
    };

    if img.is_none() {
      eprintln!("ERROR: Could not load texture image file '{filename}'.");
    }

    Self { img }
  }
}

impl Texture for ImageTexture {
  fn value(&self, u: f64, v: f64, _p: &Point3) -> Color {
    match &self.img {
      None => Color::new(1.0,0.0,1.0),
      Some(img) => {
        let u = u.clamp(0.0, 1.0);
        let v = 1.0 - v.clamp(0.0, 1.0);

        let mut i = (u * img.width() as f64) as u32;
        let mut j = (v * img.height() as f64) as u32;

        if i >= img.width() { i = img.width()-1 };
        if j >= img.height() { j = img.height()-1 };

        let color_scale = 1.0 / 255.0;
        let pixel = img.get_pixel(i, j);

        Color::new(color_scale * pixel[0] as f64, color_scale * pixel[1] as f64, color_scale * pixel[2] as f64)
      }
    }
  }
}