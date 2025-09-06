use exercises::add::add;
use once_cell::sync::Lazy;
use serde::Deserialize;

mod common;
use common::utils::load_json;

#[derive(Debug, Deserialize)]
struct AdderTestCase {
    title: String,
    a: u64,
    b: u64,
    result: u64
}

static TEST_DATA: Lazy<Vec<AdderTestCase>> =
    Lazy::new(|| load_json("tests/add.json"));

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_adder() {
    for test_case in TEST_DATA.iter() {
      println!("Probando AdderTestCase : {:?}", test_case.title);

      let result = add(test_case.a, test_case.b);
      assert_eq!(result, test_case.result);
    }
  }
}
