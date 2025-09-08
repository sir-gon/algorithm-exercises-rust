use exercises::hackerrank::warmup::simple_array_sum::simple_array_sum;
use once_cell::sync::Lazy;
use serde::Deserialize;

use crate::common;
use common::utils::load_json;

#[cfg(test)]
mod tests {
  use super::*;

  #[derive(Debug, Deserialize)]
  struct SimpleArraySumTestCase {
      input: Vec<i32>,
      expected: i32
  }

  static TEST_DATA: Lazy<Vec<SimpleArraySumTestCase>> =
    Lazy::new(|| load_json("tests/data/hackerrank/warmup/simple_array_sum.testcases.json"));

  #[test]
  fn test_simple_array_sum() {
    println!("Testing hackerrank::warmup::simple_array_sum::simple_array_sum()");

    for test_case in TEST_DATA.iter() {
      let slice: &[i32] = &test_case.input;
      let result = simple_array_sum(slice);
      assert_eq!(result, test_case.expected);
    }
  }
}
