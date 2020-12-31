pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    (1..limit)
        .filter(|n| factors.into_iter().any(|&f| f != 0 && n.rem_euclid(f) == 0))
        .sum()
}
