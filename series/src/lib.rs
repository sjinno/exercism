pub fn series(digits: &str, len: usize) -> Vec<String> {
    if len > 5 {
        return vec![];
    }

    if len == 5 {
        return vec![digits.to_string()];
    }

    (0..6 - len)
        .scan(0, |_, x| {
            let num = digits.get(x..x + len).unwrap();
            Some(num.to_string())
        })
        .collect::<Vec<String>>()
}
