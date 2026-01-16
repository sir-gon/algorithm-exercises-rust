// @link Problem definition [[docs/hackerrank/warmup/staircase.md]]

#[allow(non_snake_case)]
pub fn timeConversion(s: &str) -> String {
    let (time, period) = s.split_at(8);
    let mut time_parts: Vec<u32> = time
        .split(':')
        .map(|part| part.parse::<u32>().unwrap())
        .collect();

    if period == "AM" {
        if time_parts[0] == 12 {
            time_parts[0] = 0;
        }
    } else if time_parts[0] != 12 {
      time_parts[0] += 12;
    }

    format!("{:02}:{:02}:{:02}", time_parts[0], time_parts[1], time_parts[2])
}
