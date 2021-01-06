pub fn series(digits: &str, len: usize) -> Vec<String> {
    (0..6 - len)
        .scan(0, |_, x| {
            let num = digits.get(x..x + len).unwrap();
            Some(num.to_string())
        })
        .collect::<Vec<String>>()
}
