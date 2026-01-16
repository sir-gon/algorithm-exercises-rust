use exercises::hackerrank::warmup::time_conversion::{timeConversion};
use once_cell::sync::Lazy;
use serde::Deserialize;

use crate::common;
use common::utils::load_json;

#[cfg(test)]
mod tests {
  use super::*;

  #[derive(Debug, Deserialize)]
  struct TimeConversionTestCase {
      input: String,
      expected: String
  }

  static TEST_DATA: Lazy<Vec<TimeConversionTestCase>> =
    Lazy::new(|| load_json("tests/data/hackerrank/warmup/time_conversion.testcases.json"));

  #[test]
  fn test_time_conversion() {
    println!("Testing hackerrank::warmup::time_conversion::timeConvertion()");

    for test_case in TEST_DATA.iter() {
      let result = timeConversion(&test_case.input);
      assert_eq!(result, test_case.expected);
    }
  }
}
