use exercises::hackerrank::interview_preparation_kit::dictionaries_and_hashmaps::count_triplets_1::countTriplets;
use once_cell::sync::Lazy;
use serde::Deserialize;

use crate::common;
use common::utils::load_json;

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Deserialize)]
    struct CountTripletsTestCase {
        title: String,
        input: Vec<i32>,
        r: i32,
        expected: i64
    }

    static TEST_DATA: Lazy<Vec<CountTripletsTestCase>> = Lazy::new(|| {
        load_json(
            "tests/data/hackerrank/interview_preparation_kit/dictionaries_and_hashmaps/count_triplets_1.testcases.json",
        )
    });

    #[test]
    fn test_count_triplets() {
        println!(
            "Testing hackerrank::interview_preparation_kit::dictionaries_and_hashmaps::count_triplets_1::count_triplets()"
        );

        for test_case in TEST_DATA.iter() {
          let result = countTriplets(test_case.input.clone(), test_case.r);
          println!("Testing {} / result: {}", test_case.title,result);
          assert_eq!(result, test_case.expected);
        }
    }
}
