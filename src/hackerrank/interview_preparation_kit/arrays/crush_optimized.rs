// @link Problem definition
// [[docs/hackerrank/interview_preparation_kit/arrays/crush.md]]

#[allow(non_snake_case)]
pub fn arrayManipulation(n: i32, queries: &[Vec<i32>]) -> i64 {
  let mut result = vec![0i64; (n + 2) as usize];
  let mut maximum: i64 = 0;

  for query in queries {
    let a = query[0] as usize;
    let b = query[1] as usize;
    let k = query[2] as i64;

    // use split_at_mut to avoid simultaneous mutable borrows
    let split_pos = b + 1;
    let (left, right) = result.split_at_mut(split_pos);
    left[a] += k;
    // right[0] corresponds to result[b+1]
    right[0] -= k;
  }

  // compute prefix sums and track maximum
  let mut accum_sum: i64 = 0;
  for &value in &result {
    accum_sum += value;
    maximum = std::cmp::max(accum_sum, maximum);
  }

  maximum
}
