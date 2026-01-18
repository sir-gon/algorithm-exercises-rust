use exercises::hackerrank::projecteuler::euler002::euler002;
use once_cell::sync::Lazy;
use serde::Deserialize;

use crate::common;
use common::utils::load_json;

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Deserialize)]
    struct Euler002TestCase {
        n: i32,
        expected: i64,
    }

    static EULER002_TEST_CASES: Lazy<Vec<Euler002TestCase>> =
        Lazy::new(|| load_json("tests/data/hackerrank/projecteuler/euler002.testcases.json"));

    #[test]
    fn test_euler002() {
        println!("Testing hackerrank::projecteuler::euler002::euler002()");

        for test_case in EULER002_TEST_CASES.iter() {
            let result = euler002(test_case.n.into());
            assert_eq!(result, test_case.expected);
        }
    }
}
