use exercises::hackerrank::projecteuler::euler003::euler003;
use once_cell::sync::Lazy;
use serde::Deserialize;

use crate::common;
use common::utils::load_json;

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Deserialize)]
    struct Euler003TestCase {
        n: i64,
        expected: i64,
    }

    static TEST_DATA: Lazy<Vec<Euler003TestCase>> =
        Lazy::new(|| load_json("tests/data/hackerrank/projecteuler/euler003.testcases.json"));

    #[test]
    fn test_euler003() {
        println!("Testing hackerrank::projecteuler::euler003::euler003()");

        for test_case in TEST_DATA.iter() {
            let result = euler003(test_case.n);
            assert_eq!(
                result,
                Some(test_case.expected),
                "Failed for n = {}",
                test_case.n
            );
        }
    }

    #[test]
    fn test_euler003_edge_case() {
        const INPUT: i64 = 0;
        const EXPECTED: Option<i64> = None;
        let result = euler003(INPUT);
        assert_eq!(result, EXPECTED, "Failed for n = {}", INPUT);
    }
}
