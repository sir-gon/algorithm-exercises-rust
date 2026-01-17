use exercises::hackerrank::projecteuler::euler001::euler001;
use once_cell::sync::Lazy;
use serde::Deserialize;

use crate::common;
use common::utils::load_json;

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Deserialize)]
    struct AveryBigSumTestCase {
        a: i32,
        b: i32,
        n: i32,
        expected: i64,
    }

    static TEST_DATA: Lazy<Vec<AveryBigSumTestCase>> =
        Lazy::new(|| load_json("tests/data/hackerrank/projecteuler/euler001.testcases.json"));

    #[test]
    fn test_euler001() {
        println!("Testing hackerrank::projecteuler::euler001::euler001()");

        for test_case in TEST_DATA.iter() {
            let result = euler001(test_case.a, test_case.b, test_case.n);
            assert_eq!(result, test_case.expected);
        }
    }
}
