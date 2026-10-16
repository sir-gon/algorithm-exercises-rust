// @link Problem definition
// [[docs/hackerrank/interview_preparation_kit/dictionaries_and_hashmaps/count_triplets_1.md]]

// HackerRank doesn't have a Rust environment for this problem.

use std::collections::HashMap;

#[allow(non_snake_case)]
pub fn countTriplets(list: Vec<i32>, ratio: i32) -> i64 {

  let mut aCounter = HashMap::new();
  let mut bCounter = HashMap::new();
  let mut triplets = 0;


  for &item in &list {
    if let std::collections::hash_map::Entry::Vacant(e) = aCounter.entry(item) {
      e.insert(1);
    } else {
      *aCounter.get_mut(&item).unwrap() += 1;
    }
  }

  let mut prevItemCount: i64;
  let mut nextItemCount: i64;

  for &item in &list {
    let j = item / ratio;
    let k = item * ratio;

    aCounter.entry(item).and_modify(|e| *e -= 1);

    prevItemCount = *bCounter.get(&j).unwrap_or(&0);
    nextItemCount = *aCounter.get(&k).unwrap_or(&0);

    if item % ratio == 0 {
      triplets += prevItemCount * nextItemCount;
    }

    bCounter.entry(item).and_modify(|e| *e += 1).or_insert(1);
  }

  triplets
}
