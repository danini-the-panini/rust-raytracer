use crate::{vec3::Color};

pub fn write_color(pixel_color: Color) {
  let r = pixel_color.x().sqrt();
  let g = pixel_color.y().sqrt();
  let b = pixel_color.z().sqrt();

  println!("{} {} {}",
    (256.0 * r.clamp(0.0, 0.999)) as i32,
    (256.0 * g.clamp(0.0, 0.999)) as i32,
    (256.0 * b.clamp(0.0, 0.999)) as i32
  );
}