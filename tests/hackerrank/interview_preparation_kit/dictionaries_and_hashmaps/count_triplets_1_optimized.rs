use exercises::hackerrank::interview_preparation_kit::dictionaries_and_hashmaps::count_triplets_1_optimized::countTriplets;
use once_cell::sync::Lazy;
use serde::Deserialize;

use crate::common;
use common::utils::load_json;

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Deserialize, Clone)]
    struct CountTripletsTestCase {
        title: String,
        input: Vec<i32>,
        r: i32,
        expected: i64,
    }

    static TEST_DATA: Lazy<Vec<CountTripletsTestCase>> = Lazy::new(|| {
        load_json(
            "tests/data/hackerrank/interview_preparation_kit/dictionaries_and_hashmaps/count_triplets_1.testcases.json",
        )
    });

    static TEST_DATA_BIG_CASES: Lazy<Vec<CountTripletsTestCase>> = Lazy::new(|| {
        load_json(
            "tests/data/hackerrank/interview_preparation_kit/dictionaries_and_hashmaps/count_triplets_1.big.testcases.json",
        )
    });

    #[test]
    fn test_count_triplets() {
        println!(
            "Testing hackerrank::interview_preparation_kit::dictionaries_and_hashmaps::count_triplets_1::count_triplets()"
        );

        let mut all_test_cases: Vec<CountTripletsTestCase> = TEST_DATA.iter().cloned().collect();
        all_test_cases.extend(TEST_DATA_BIG_CASES.iter().cloned());

        for test_case in all_test_cases {
            let result = countTriplets(test_case.input.clone(), test_case.r);
            println!("Testing {} / result: {}", test_case.title, result);
            assert_eq!(result, test_case.expected);
        }
    }
}
