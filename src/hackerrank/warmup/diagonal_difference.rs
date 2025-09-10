// @link Problem definition [[docs/hackerrank/warmup/diagonal_difference.md]]

pub fn diagonal_difference(arr: &[Vec<i32>]) -> i32 {
  let mut diag1 = 0;
  let mut diag2 = 0;

  let last = arr.len() - 1;
  for (i, row) in arr.iter().enumerate() {
    for(j, &value) in row.iter().enumerate() {
      if i == j {
        diag1 += value;
        diag2 += arr[last - i][j];
      }
    }
  }

  (diag1 - diag2).abs()
}

