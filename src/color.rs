use crate::vec3::Color;

pub fn write_color(pixel_color: Color) {
  let r = (255.999 * pixel_color.x()) as u16;
  let g = (255.999 * pixel_color.y()) as u16;
  let b = (255.999 * pixel_color.z()) as u16;
  println!("{r} {g} {b}");
}