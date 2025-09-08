use exercises::hackerrank::warmup::solve_me_first::solve_me_first as add;
use once_cell::sync::Lazy;
use serde::Deserialize;

use crate::common;
use common::utils::load_json;



#[derive(Debug, Deserialize)]
struct SolveMeFirstTestCase {
    title: String,
    a: i32,
    b: i32,
    result: i32
}

static TEST_DATA: Lazy<Vec<SolveMeFirstTestCase>> =
    Lazy::new(|| load_json("tests/data/hackerrank/warmup/add.json"));

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_solve_me_first() {
    for test_case in TEST_DATA.iter() {
      println!("Testing hackerrank::warmup::solve_me_first::solve_me_first() : {:?}", test_case.title);

      let result = add(test_case.a, test_case.b);
      assert_eq!(result, test_case.result);
    }
  }
}
