// @link Problem definition [[docs/hackerrank/projecteuler/euler003.md]]
// @link Solution notes [[docs/hackerrank/projecteuler/euler003-solution-notes.md]]

pub fn euler003(n: i64) -> Option<i64> {
    if n < 2 {
        return None;
    }

    let mut divisor = n;
    let mut max_prime_factor: Option<i64> = None;

    let mut i = 2;
    while i <= (divisor as f64).sqrt() as i64 {
        if 0 == divisor % i {
            divisor /= i;
            max_prime_factor = Some(divisor);
        }

        i += 1;
    }

    if max_prime_factor.is_none() {
        max_prime_factor = Some(n);
    }

    Some(max_prime_factor.unwrap_or(n))
}
