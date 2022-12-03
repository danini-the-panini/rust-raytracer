pub fn random_double() -> f64 {
  // Returns a random real in [0,1).
  rand::random::<f64>()
}

pub fn random_double_in_range(min: f64, max: f64) -> f64 {
  // Returns a random real in [min,max).
  min + (max-min)*random_double()
}
pub fn divide_into_parts(n: i32, m: i32) -> Vec<i32> {
  (0..m).map(|i| (n / m as i32) + ((i + 1) <= (n % m as i32)) as i32).collect()
}
