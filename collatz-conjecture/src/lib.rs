pub fn collatz(n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }

    if n == 1 {
        return Some(0);
    }

    let mut num = n.to_owned();
    (0..)
        .scan(1, |s, _| {
            if num % 2 == 0 {
                num /= 2;
            } else {
                num = num * 3 + 1;
            }
            *s += 1;
            if num != 1 {
                Some(*s)
            } else {
                None
            }
        })
        .last()
}
