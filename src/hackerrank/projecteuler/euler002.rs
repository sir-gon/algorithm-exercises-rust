// @link Problem definition [[docs/hackerrank/projecteuler/euler002.md]]

pub fn euler002(n: i64) -> i64 {
    let mut total = 0;
    let mut fibo1 = 1;
    let mut fibo2 = 1;

    while fibo1 + fibo2 < n {
        let fibo = fibo1 + fibo2;
        fibo1 = fibo2;
        fibo2 = fibo;

        if fibo % 2 == 0 {
            total += fibo;
        }
    }

    total
}
