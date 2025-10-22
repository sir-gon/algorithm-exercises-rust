// @link Problem definition [[docs/hackerrank/warmup/mini_max_sum.md]]

pub fn mini_max_sum(arr: &[i32]) -> String {
  if arr.is_empty() {
    panic!("Empty input");
  }

  let mut tsum = 0;
  let mut tmin: i32 = arr[0];
  let mut tmax: i32 = arr[1];

  for value in arr.iter() {
    tsum += *value;

    if *value < tmin {
      tmin = *value;
    }

    if *value > tmax {
      tmax = *value;
    }
  }

  let result = format!("{} {}", tsum - tmax, tsum - tmin);

  result
}

#[allow(non_snake_case)]
pub fn miniMaxSum(arr: &[i32]) {
  print!("{}", mini_max_sum(arr))
}
