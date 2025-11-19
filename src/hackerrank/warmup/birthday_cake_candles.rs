// @link Problem definition [[docs/hackerrank/warmup/a_very_big_sum.md]]



#[allow(non_snake_case)]
pub fn birthdayCakeCandles(candles: &[i32]) -> i32 {

  let mut counter: i32 = 0;
  let mut maximum: &i32 = &candles[0];

  for element  in candles.iter() {
    if element > maximum
    {
      maximum = element;
      counter = 1;
    } else if element == maximum {
      counter += 1;
    }
  }

  counter
}

