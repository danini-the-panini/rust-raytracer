mod vec3;
mod color;
use crate::vec3::{Vec3, Color};
use crate::color::write_color;

fn main() {
  // Image

  let image_width = 256;
  let image_height = 256;

  //Render
  println!("P3\n{image_width} {image_height}\n255");

  for j in (0..image_height).rev() {
    eprint!("\rScanlines remaining: {j} ");
    for i in 0..image_width {
      let pixel_color = Color::new(
        (i as f64) / (image_width as f64 - 1.0),
        (j as f64) / (image_height as f64 - 1.0),
        0.25
      );
      write_color(pixel_color);
    }
  }
  eprintln!("\nDone")
}