// @link Problem definition [[docs/hackerrank/warmup/mini_max_sum.md]]

pub fn mini_max_sum(arr: &[i32]) -> String {
  if arr.is_empty() {
    panic!("Empty input");
  }

  let mut tsum: i64 = 0;
  let mut tmin: i64 = arr[0].into();
  let mut tmax: i64 = arr[1].into();
  let mut tvalue: i64;

  for value in arr.iter() {
    tvalue = *value as i64;
    tsum += tvalue;

    if tvalue < tmin {
      tmin = tvalue;
    }

    if tvalue > tmax {
      tmax = tvalue;
    }
  }

  let result = format!("{} {}", tsum - tmax, tsum - tmin);

  result
}

#[allow(non_snake_case)]
pub fn miniMaxSum(arr: &[i32]) {
  print!("{}", mini_max_sum(arr))
}
