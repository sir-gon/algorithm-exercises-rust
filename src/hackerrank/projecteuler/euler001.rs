// Function to find sum of Arithmetic Progression series
pub fn sum_of_arithmetic_progression(n: i64, d: i64) -> i64 {
    // Number of terms
    let n = n / d;

    (n) * (1 + n) * d / 2
}

// Function to find the sum of all multiples of a and b below n
pub fn euler001(a: i32, b: i32, n: i32) -> i64 {
    // Since, we need the sum of multiples less than N
    let n = n - 1;
    let lcm = (a as i64 * b as i64) / gcd(a, b) as i64;

    sum_of_arithmetic_progression(n as i64, a as i64)
        + sum_of_arithmetic_progression(n as i64, b as i64)
        - sum_of_arithmetic_progression(n as i64, lcm)
}

fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}
