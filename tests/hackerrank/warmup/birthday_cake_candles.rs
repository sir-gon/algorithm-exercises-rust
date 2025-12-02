use exercises::hackerrank::warmup::birthday_cake_candles::birthdayCakeCandles;
use once_cell::sync::Lazy;
use serde::Deserialize;

use crate::common;
use common::utils::load_json;

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Deserialize)]
    struct BirthdayCakeCandlesTestCase {
        input: Vec<i32>,
        expected: i32,
    }

    static TEST_DATA: Lazy<Vec<BirthdayCakeCandlesTestCase>> = Lazy::new(|| {
        load_json("tests/data/hackerrank/warmup/birthday_cake_candles.testcases.json")
    });

    #[test]
    fn test_birthday_cake_candles() {
        println!("Testing hackerrank::warmup::birthday_cake_candles::birthdayCakeCandles()");

        for test_case in TEST_DATA.iter() {
            let result = birthdayCakeCandles(test_case.input.as_slice());
            assert_eq!(result, test_case.expected);
        }
    }
}
