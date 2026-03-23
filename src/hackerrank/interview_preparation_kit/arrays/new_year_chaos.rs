// @link Problem definition [[docs/hackerrank/interview_preparation_kit/arrays/new_year_chaos.md]]

#[allow(non_snake_case)]
pub fn minimumBribesCalculate(queue: &[i32]) -> String  {
    let mut bribes = 0;

    for (i, &value) in queue.iter().enumerate() {
        let position = i as i32 + 1;

        if value - position > 2 {
            return "Too chaotic".into();
        }
        for k in (value - 2).max(0)..position as i32 {
            if queue[k as usize] > value {
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


