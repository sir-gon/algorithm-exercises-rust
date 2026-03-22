use exercises::hackerrank::interview_preparation_kit::arrays::new_year_chaos::*;
use once_cell::sync::Lazy;
use serde::Deserialize;

use crate::common;
use common::utils::load_json;

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Deserialize)]
    struct MinimumBribesTestCase {
        input: Vec<i32>,
        expected: String,
    }

    static TEST_DATA: Lazy<Vec<MinimumBribesTestCase>> = Lazy::new(|| {
        load_json(
            "tests/data/hackerrank/interview_preparation_kit/arrays/new_year_chaos.testcases.json",
        )
    });

    #[test]
    fn test_minimum_bribes() {
        println!(
            "Testing hackerrank::interview_preparation_kit::arrays::new_year_chaos::minimumBribes()"
        );

        for test_case in TEST_DATA.iter() {
            let result = minimumBribesCalculate(test_case.input.as_slice());
            assert_eq!(result, test_case.expected);
        }
    }
}
