// @link Problem definition [[docs/hackerrank/warmup/compare_triplets.md]]

pub fn compare_triplets(a: &[i32], b: &[i32]) -> Vec<i32> {
  let mut awards: Vec<i32> = vec![0, 0];

  for (i, value) in a.iter().enumerate() {
    if *value > b[i] {
      awards[0] += 1;
    } else if *value < b[i] {
      awards[1] += 1;
    }
  }

  awards
}
