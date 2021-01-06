pub fn collatz(n: u64) -> Option<u64> {
    match n {
        0 => None,
        1 => Some(0),
        _ => {
            let mut num = n;
            (0..)
                .scan(1, |s, _| {
                    match num.rem_euclid(2) {
                        0 => num /= 2,
                        _ => num = num * 3 + 1,
                    }
                    *s += 1;
                    match num {
                        1 => None,
                        _ => Some(*s),
                    }
                })
                .last()
        }
    }
}

// // Community solution using recursion.
// pub fn collatz(n: u64) -> Option<u64> {
//     match n {
//         0 => None,
//         1 => Some(0),
//         n if n % 2 == 0 => collatz(n / 2).map(|x| x + 1),
//         n => collatz(3 * n + 1).map(|x| x + 1),
//     }
// }
