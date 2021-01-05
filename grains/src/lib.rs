pub fn square(s: u32) -> u64 {
    match s {
        num if num == 0 || num > 64 => panic!("Square must be between 1 and 64"),
        _ => 2u64.pow(s - 1),
    }
}

pub fn total() -> u64 {
    (0..64).into_iter().fold(0, |acc, x| acc + 2u64.pow(x))
}
