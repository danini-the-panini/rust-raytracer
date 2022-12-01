fn main() {
  // Image

  let image_width = 256;
  let image_height = 256;

  //Render
  println!("P3\n{image_width} {image_height}\n255");

  for j in (0..image_height).rev() {
    eprint!("\rScanlines remaining: {j} ");
    for i in 0..image_width {
      let r = (i as f64) / (image_width as f64 - 1.0);
      let g = (j as f64) / (image_height as f64 - 1.0);
      let b = 0.25;
      
      let ir = (255.999 * r) as u16;
      let ig = (255.999 * g) as u16;
      let ib = (255.999 * b) as u16;

      println!("{ir} {ig} {ib}");
    }
  }
  eprintln!()
}