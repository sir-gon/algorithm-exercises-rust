use exercises::hackerrank::warmup::mini_max_sum::*;
use once_cell::sync::Lazy;
use serde::Deserialize;

use crate::common;
use common::utils::load_json;
use log::{warn};

#[cfg(test)]
mod tests {
  use super::*;

  #[derive(Debug, Deserialize)]
  struct MiniMaxSumTest {
      input: Vec<i32>,
      expected: String,
      title: String
  }

  static TEST_DATA: Lazy<Vec<MiniMaxSumTest>> =
    Lazy::new(|| load_json("tests/data/hackerrank/warmup/mini_max_sum.testcases.json"));

  #[test]
  fn test_mini_max_sum() {
    println!("Testing hackerrank::warmup::mini_max_sum::mini_max_sum()");

    for test_case in TEST_DATA.iter() {
      let slice: &[i32] = &test_case.input;
      let result = mini_max_sum(slice);
      // miniMaxSum(slice);

      warn!("miniMaxSum ({}) result => {}" , test_case.title, result);

      assert_eq!(result, test_case.expected);
    }
  }

  #[test]
  fn test_mini_max_sum_edge_case() {
    println!("Testing hackerrank::warmup::mini_max_sum::mini_max_sum()");

    let input: Vec<i32> = vec![];
    let slice: &[i32] = &input;
    let result = std::panic::catch_unwind(|| mini_max_sum(slice));
    assert!(result.is_err());
  }
}
