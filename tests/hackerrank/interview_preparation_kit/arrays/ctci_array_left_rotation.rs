use exercises::hackerrank::interview_preparation_kit::arrays::ctci_array_left_rotation::rotLeft;
use once_cell::sync::Lazy;
use serde::Deserialize;

use crate::common;
use common::utils::load_json;

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Deserialize)]
    struct RotLeftTestCase {
        input: Vec<i32>,
        d_rotations: i32,
        expected: Vec<i32>
    }

    static TEST_DATA: Lazy<Vec<RotLeftTestCase>> = Lazy::new(|| {
        load_json(
            "tests/data/hackerrank/interview_preparation_kit/arrays/ctci_array_left_rotation.testcases.json",
        )
    });

    #[test]
    fn test_rot_left() {
        println!(
            "Testing hackerrank::interview_preparation_kit::arrays::ctci_array_left_rotation::rotLeft()"
        );

        for test_case in TEST_DATA.iter() {
            let result = rotLeft(&test_case.input, test_case.d_rotations);
            assert_eq!(result, test_case.expected);
        }
    }
}
