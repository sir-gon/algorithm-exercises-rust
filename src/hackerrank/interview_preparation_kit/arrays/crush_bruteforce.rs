// @link Problem definition
// [[docs/hackerrank/interview_preparation_kit/arrays/crush.md]]

#[allow(non_snake_case)]
pub fn arrayManipulation(n: i32, queries: &[Vec<i32>]) -> i64 {
  let mut result = vec![0; (n + 1) as usize];
  let mut maximum: i64 = 0;

  for query in queries {
    let a = query[0] as usize;
    let b = query[1] as usize;
    let k = query[2] as i64;

    for i in a..=b {
      result[i] += k;
    }
  }

  for value in result {
    maximum = std::cmp::max(value, maximum);
  }

  maximum
}
