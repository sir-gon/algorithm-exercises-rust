use exercises::hackerrank::interview_preparation_kit::dictionaries_and_hashmaps::ctci_ransom_note::{checkMagazine, checkMagazineText};
use once_cell::sync::Lazy;
use serde::Deserialize;

use crate::common;
use common::utils::load_json;

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Deserialize)]
    struct CheckMagazineTestCase {
        title: String,
        magazine: Vec<String>,
        note: Vec<String>,
        expected: String
    }

    static TEST_DATA: Lazy<Vec<CheckMagazineTestCase>> = Lazy::new(|| {
        load_json(
            "tests/data/hackerrank/interview_preparation_kit/dictionaries_and_hashmaps/ctci_ransom_note.testcases.json",
        )
    });

    #[test]
    fn test_check_magazine() {
        println!(
            "Testing hackerrank::interview_preparation_kit::dictionaries_and_hashmaps::ctci_ransom_note::checkMagazine()"
        );

        for test_case in TEST_DATA.iter() {
          let result = checkMagazineText(&test_case.magazine, &test_case.note);
          println!("Testing {} / result: {}", test_case.title,result);

          checkMagazine(&test_case.magazine, &test_case.note);
          assert_eq!(result, test_case.expected);
        }
    }
}
