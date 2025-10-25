// @link Problem definition [[docs/hackerrank/warmup/mini_max_sum.md]]

#[cfg(not(test))]
use log::{info}; // Use log crate when building application

#[cfg(test)]
use std::{println as info}; // Workaround to use prinltn! for logs.

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

#[allow(non_snake_case, reason = "Keeping original interface from HackerRank")]
pub fn miniMaxSum(arr: &[i32]) {
  let result = mini_max_sum(arr);

  info!("miniMaxSum result => {}" , result);
  println!("{}", mini_max_sum(arr))
}
