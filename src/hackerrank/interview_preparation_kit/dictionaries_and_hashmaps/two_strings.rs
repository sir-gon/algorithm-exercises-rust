// @link Problem definition
// [[docs/hackerrank/interview_preparation_kit/dictionaries_and_hashmaps/two_strings.md]]

#[allow(non_snake_case)]
pub fn twoStrings(s1: &str, s2: &str) -> String {
    for c in s1.chars() {
        if s2.contains(c) {
            return String::from("YES");
        }
    }
    String::from("NO")
}
