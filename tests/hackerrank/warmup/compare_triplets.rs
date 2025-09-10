use exercises::hackerrank::warmup::compare_triplets::compare_triplets;
use once_cell::sync::Lazy;
use serde::Deserialize;

use crate::common;
use common::utils::load_json;

#[cfg(test)]
mod tests {
  use super::*;

  #[derive(Debug, Deserialize)]
  struct CompareTripletsTestCase {
      a: Vec<i32>,
      b: Vec<i32>,
      expected: Vec<i32>
  }

  static TEST_DATA: Lazy<Vec<CompareTripletsTestCase>> =
      Lazy::new(|| load_json("tests/data/hackerrank/warmup/compare_triplets.testcases.json"));

  #[test]
  fn test_compare_triplets() {
    for test_case in TEST_DATA.iter() {
      println!("Testing hackerrank::warmup::compare_triplets::compare_triplets()");

      let result = compare_triplets(test_case.a.as_slice(), test_case.b.as_slice());
      assert_eq!(result, test_case.expected);
    }
  }
}
