// @link Problem definition [[docs/hackerrank/warmup/solveMeFirst.md]]

pub fn plus_minus_calculate(arr: &[i32]) -> (f64, f64, f64) {
  let n = arr.len() as f64;
  let (mut pos, mut neg, mut zero) = (0, 0, 0);

  for &num in arr {
      if num > 0 {
          pos += 1;
      } else if num < 0 {
          neg += 1;
      } else {
          zero += 1;
      }
  }

  (pos as f64 / n, neg as f64 / n, zero as f64 / n)
}

pub fn plus_minus_calculate_string(arr: &[i32]) -> (String, String, String) {
  let (pos, neg, zero) = plus_minus_calculate(arr);
  (format!("{:.6}", pos, ), format!("{:.6}", neg), format!("{:.6}", zero))
}

pub fn plus_minus(arr: &[i32]) {
  let (pos, neg, zero) = plus_minus_calculate_string(arr);

  println!("{}", pos);
  println!("{}", neg);
  println!("{}", zero);
}
