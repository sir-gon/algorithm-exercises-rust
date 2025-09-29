// @link Problem definition [[docs/hackerrank/warmup/staircase.md]]

pub fn staircase_string(n: i32) -> String {
  let mut result: Vec<String> = Vec::new();

  for i in 1..=n {
    let spaces = " ".repeat((n - i) as usize);
    let hashes = "#".repeat(i as usize);

    result.push(format!("{}{}", spaces, hashes));
  }

  result.join("\n")
}

pub fn staircase(n: i32) {
  println!("{}", staircase_string(n));
}
