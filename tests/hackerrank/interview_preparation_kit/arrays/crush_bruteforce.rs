use exercises::hackerrank::interview_preparation_kit::arrays::crush_bruteforce::arrayManipulation;
use once_cell::sync::Lazy;
use serde::Deserialize;

use crate::common;
use common::utils::load_json;

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Deserialize)]
    struct ArrayManipulationTestCase {
        n: i32,
        queries: Vec<Vec<i32>>,
        expected: i64
    }

    static TEST_DATA: Lazy<Vec<ArrayManipulationTestCase>> = Lazy::new(|| {
        load_json(
            "tests/data/hackerrank/interview_preparation_kit/arrays/crush.testcases.json",
        )
    });

    #[test]
    fn test_array_manipulation() {
        println!(
            "Testing hackerrank::interview_preparation_kit::arrays::arrayManipulation()"
        );

        for test_case in TEST_DATA.iter() {
            let result = arrayManipulation(test_case.n, &test_case.queries);
            assert_eq!(result, test_case.expected);
        }
    }
}
