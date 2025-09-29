use exercises::hackerrank::warmup::staircase::{staircase, staircase_string};
use once_cell::sync::Lazy;
use serde::Deserialize;

use crate::common;
use common::utils::load_json;

#[cfg(test)]
mod tests {
  use super::*;

  #[derive(Debug, Deserialize)]
  struct AveryBigSumTestCase {
      input: i32,
      expected: String
  }

  static TEST_DATA: Lazy<Vec<AveryBigSumTestCase>> =
    Lazy::new(|| load_json("tests/data/hackerrank/warmup/staircase.testcases.json"));

  #[test]
  fn test_staircase() {
    println!("Testing hackerrank::warmup::staircase::staircase()");

    for test_case in TEST_DATA.iter() {
      let result = staircase_string(test_case.input);
      staircase(test_case.input);
      assert_eq!(result, test_case.expected);
    }
  }
}
