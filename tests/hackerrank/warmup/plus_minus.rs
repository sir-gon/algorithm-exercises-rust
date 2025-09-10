use serde::Deserialize;
use once_cell::sync::Lazy;

use exercises::hackerrank::warmup::plus_minus::plus_minus;
use exercises::hackerrank::warmup::plus_minus::plus_minus_calculate_string;
use crate::common::utils::load_json;

#[cfg(test)]
mod tests {
  use super::*;

  #[derive(Debug, Deserialize)]
  struct PlusMinusTestCase {
    title: String,
    input: Vec<i32>,
    expected: Vec<String>,
  }

  static TEST_DATA: Lazy<Vec<PlusMinusTestCase>> =
      Lazy::new(|| load_json("tests/data/hackerrank/warmup/plus_minus.testcases.json"));

  #[test]
  fn test_plus_minus() {
    for test_case in TEST_DATA.iter() {
      println!("Testing hackerrank::warmup::plus_minus::plus_minus() : {:?}", test_case.title);

      let expected: (_, _, _) = (
        test_case.expected[0].clone(),
        test_case.expected[1].clone(),
        test_case.expected[2].clone()
      );
      let result = plus_minus_calculate_string(test_case.input.as_slice());
      assert_eq!(result, expected);

      // call printer for coverage
      plus_minus(test_case.input.as_slice());
    }
  }
}
