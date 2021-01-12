// pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
//     match upper_bound {
//         bound if bound < 2 => vec![],
//         _ => {
//             let mut sieve: Vec<u64> = (2..upper_bound + 1).collect();
//             let mut idx = 0;
//             while let Some(num) = sieve.get(idx) {
//                 match num {
//                     0 => {
//                         idx += 1;
//                         continue;
//                     }
//                     _ => {
//                         let p = *num;
//                         sieve.iter_mut().for_each(|x| {
//                             if *x != p && *x % p == 0 {
//                                 *x = 0;
//                             }
//                         });
//                         idx += 1;
//                     }
//                 }
//             }
//             sieve.into_iter().filter(|x| x != &0).collect()
//         }
//     }
// }

// Community solution 1
// #![warn(clippy::pedantic)]
pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut numbers: Vec<_> = (2..=upper_bound).map(Option::from).collect();
    (0..numbers.len())
        .filter_map(|i| {
            let prime = numbers[i].take()?;
            (prime..=upper_bound)
                .step_by(prime as usize)
                .skip(1)
                .for_each(|i| numbers[(i - 2) as usize] = None);
            Some(prime)
        })
        .collect()
}
