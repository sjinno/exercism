// pub fn factors(n: u64) -> Vec<u64> {
//     if n == 1 {
//         return vec![];
//     }
//     let mut n = n.to_owned();
//     let mut d = 2;
//     let mut ans = vec![];

//     loop {
//         if n.div_euclid(d) == 1 && n.rem_euclid(d) == 0 {
//             ans.push(d);
//             break;
//         }
//         if n.rem_euclid(d) == 0 {
//             n /= d;
//             ans.push(d);
//             continue;
//         }
//         d += 1;
//     }

//     ans
// }

pub fn factors(mut n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    let mut candidates = 2..;

    while n > 1 {
        let x = candidates.next().unwrap();

        while n % x == 0 {
            n /= x;
            factors.push(x);
        }
    }

    factors
}
