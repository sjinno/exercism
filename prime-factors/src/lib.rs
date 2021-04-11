pub fn factors(mut n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    let mut candidates = 2..;
    while n != 1 {
        if let Some(x) = candidates.next() {
            while n % x == 0 {
                n /= x;
                factors.push(x);
            }
        }
    }
    factors
}
