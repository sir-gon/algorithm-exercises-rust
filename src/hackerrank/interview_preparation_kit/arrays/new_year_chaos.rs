// @link Problem definition [[docs/hackerrank/interview_preparation_kit/arrays/new_year_chaos.md]]

#[allow(non_snake_case)]
pub fn minimumBribesCalculate(q: &[i32]) -> String  {
    let mut bribes = 0;
    for (i, &p) in q.iter().enumerate() {
        if p - (i as i32 + 1) > 2 {
            return "Too chaotic".into();
        }
        for j in (p - 2).max(0)..i as i32 {
            if q[j as usize] > p {
                bribes += 1;
            }
        }
    }

    bribes.to_string()
}

#[allow(non_snake_case)]
pub fn minimumBribes(q: &[i32]) {
  println!("{}", minimumBribesCalculate(q));
}


