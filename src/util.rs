pub fn random_double() -> f64 {
  // Returns a random real in [0,1).
  rand::random::<f64>()
}
pub fn random_int(min: i32, max: i32) -> i32 {
  random_double_in_range(min as f64, (max+1) as f64) as i32
}

pub fn random_double_in_range(min: f64, max: f64) -> f64 {
  // Returns a random real in [min,max).
  min + (max-min)*random_double()
}
pub fn divide_into_parts(n: i32, m: i32) -> Vec<i32> {
  (0..m).map(|i| (n / m as i32) + ((i + 1) <= (n % m as i32)) as i32).collect()
}

pub fn min(a: f64, b: f64) -> f64 {
  if a < b { a } else { b }
}
pub fn max(a: f64, b: f64) -> f64 {
  if a > b { a } else { b }
}
