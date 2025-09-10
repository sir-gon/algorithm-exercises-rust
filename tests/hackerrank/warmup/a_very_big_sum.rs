use exercises::hackerrank::warmup::a_very_big_sum::a_very_big_sum;
use once_cell::sync::Lazy;
use serde::Deserialize;

use crate::common;
use common::utils::load_json;

#[cfg(test)]
mod tests {
  use super::*;

  #[derive(Debug, Deserialize)]
  struct AveryBigSumTestCase {
      input: Vec<i64>,
      expected: i64
  }

  static TEST_DATA: Lazy<Vec<AveryBigSumTestCase>> =
    Lazy::new(|| load_json("tests/data/hackerrank/warmup/a_very_big_sum.testcases.json"));

  #[test]
  fn test_a_very_big_sum() {
    println!("Testing hackerrank::warmup::a_very_big_sum::a_very_big_sum()");

    for test_case in TEST_DATA.iter() {
      let slice: &[i64] = &test_case.input;
      let result = a_very_big_sum(slice);
      assert_eq!(result, test_case.expected);
    }
  }
}
