use exercises::hackerrank::interview_preparation_kit::dictionaries_and_hashmaps::two_strings::{twoStrings};
use once_cell::sync::Lazy;
use serde::Deserialize;

use crate::common;
use common::utils::load_json;

#[cfg(test)]
mod tests {

use super::*;

    #[derive(Debug, Deserialize)]
    struct TwoStringsTestCase {
        title: String,
        s1: String,
        s2: String,
        expected: String
    }

    static TEST_DATA: Lazy<Vec<TwoStringsTestCase>> = Lazy::new(|| {
        load_json(
            "tests/data/hackerrank/interview_preparation_kit/dictionaries_and_hashmaps/two_strings.testcases.json",
        )
    });

    #[test]
    fn test_two_strings() {
        println!(
            "Testing hackerrank::interview_preparation_kit::dictionaries_and_hashmaps::two_strings::twoStrings()"
        );

        for test_case in TEST_DATA.iter() {
          let result = twoStrings(&test_case.s1, &test_case.s2);
          println!("Testing {} / result: {}", test_case.title,result);

          assert_eq!(result, test_case.expected);
        }
    }
}
