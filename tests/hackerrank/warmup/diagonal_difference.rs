use exercises::hackerrank::warmup::diagonal_difference::diagonal_difference;
use once_cell::sync::Lazy;
use serde::Deserialize;

use crate::common;
use common::utils::load_json;

#[cfg(test)]
mod tests {
  use super::*;

  #[derive(Debug, Deserialize)]
  struct DiagonalDifferenceTestCase {
      matrix: Vec<Vec<i32>>,
      expected: i32
  }

  static TEST_DATA: Lazy<Vec<DiagonalDifferenceTestCase>> =
    Lazy::new(|| load_json("tests/data/hackerrank/warmup/diagonal_difference.testcases.json"));

  #[test]
  fn test_diagonal_difference() {
    println!("Testing hackerrank::warmup::diagonal_difference::diagonal_difference()");

    for test_case in TEST_DATA.iter() {
      let slice: &[Vec<i32>] = &test_case.matrix;
      let result = diagonal_difference(slice);
      assert_eq!(result, test_case.expected);
    }
  }
}
