// @link Problem definition [[docs/hackerrank/projecteuler/euler003.md]]
// @link Solution notes [[docs/hackerrank/projecteuler/euler003-solution-notes.md]]

pub fn euler003(n: i64) -> i64 {
    if n < 2 {
        return 0;
    }

    let mut divisor = n;
    let mut max_prime_factor = None;

    let mut i = 2;
    while i * i <= divisor {
        if 0 == divisor % i {
            while divisor % i == 0 {
                divisor /= i;
            }
            max_prime_factor = Some(i);
        }
        i += 1;
    }

    if divisor > 1 {
        return divisor;
    }

    max_prime_factor.unwrap_or(n)
}
