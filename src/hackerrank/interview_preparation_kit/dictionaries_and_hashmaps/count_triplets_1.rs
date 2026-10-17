// @link Problem definition
// [[docs/hackerrank/interview_preparation_kit/dictionaries_and_hashmaps/count_triplets_1.md]]

// HackerRank doesn't have a Rust environment for this problem.

#[allow(non_snake_case)]
pub fn countTriplets(list: Vec<i32>, ratio: i32) -> i64 {
  let size = list.len();

  let mut counter: i64 = 0;

  for i in 0..size.saturating_sub(2) {
    for j in i +1 ..size.saturating_sub(1) {
      for k in j +1 ..size {
        if ratio * list[i] == list[j] && ratio * list[j] == list[k] {
          counter += 1
        }
      }
    }
  }

  counter
}
