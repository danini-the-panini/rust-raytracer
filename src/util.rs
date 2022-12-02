pub fn random_double() -> f64 {
  rand::random::<f64>()
}

pub fn random_double_in_range(min: f64, max: f64) -> f64 {
  min + (max-min)*random_double()
}