// @link Problem definition
// [[docs/hackerrank/interview_preparation_kit/dictionaries_and_hashmaps/ctci_ransom_note.md]]

// HackerRank doesn't have a Rust environment for this problem.

use std::collections::HashMap;

const YES: &str = "Yes";
const NO: &str = "No";

#[allow(non_snake_case)]
fn checkMagazineCompute(magazine: &[String], note: &[String]) -> bool {
    let mut dictionary = HashMap::<String, i64>::new();

    for word in magazine {
        let count = dictionary.entry(word.clone()).or_insert(0);
        *count += 1;
    }

    for word in note {
        let count = dictionary.entry(word.clone()).or_insert(0);
        *count -= 1;
        if *count < 0 {
            return false;
        }
    }

    true
}

#[allow(non_snake_case)]
pub fn checkMagazineText(magazine: &[String], note: &[String]) -> String {
    if checkMagazineCompute(magazine, note) {
        YES.to_string()
    } else {
        NO.to_string()
    }
}

#[allow(non_snake_case)]
pub fn checkMagazine(magazine: &[String], note: &[String]) {
  print!("{}", checkMagazineText(magazine, note));
}
