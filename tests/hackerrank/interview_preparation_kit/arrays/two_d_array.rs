use exercises::hackerrank::interview_preparation_kit::arrays::two_d_array::hourglassSum;
use once_cell::sync::Lazy;
use serde::Deserialize;

use crate::common;
use common::utils::load_json;

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Deserialize)]
    struct HourglassSumTestCase {
        arr: Vec<Vec<i32>>,
        expected: i32,
    }

    static TEST_DATA: Lazy<Vec<HourglassSumTestCase>> = Lazy::new(|| {
        load_json(
            "tests/data/hackerrank/interview_preparation_kit/arrays/two_d_array.testcases.json",
        )
    });

    #[test]
    fn test_hourglass_sum() {
        println!(
            "Testing hackerrank::interview_preparation_kit::arrays::two_d_array::hourglassSum()"
        );

        for test_case in TEST_DATA.iter() {
            let result = hourglassSum(test_case.arr.as_slice());
            assert_eq!(result, test_case.expected);
        }
    }
}
