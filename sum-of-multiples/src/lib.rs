// use std::collections::HashSet;

// pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
//     let length = factors.len();
//     if length == 0 {
//         return 0;
//     }

//     let mut visited_nums: HashSet<u32> = HashSet::new();
//     let mut sum = 0;
//     for num in factors {
//         if num == &0 {
//             continue;
//         }
//         let mut n = *num;
//         while n < limit {
//             if !visited_nums.contains(&n) {
//                 visited_nums.insert(n);
//                 sum += n;
//             }
//             n += num;
//         }
//     }
//     sum
// }

#![feature(test)]

extern crate test;
use test::Bencher;

use std::collections::HashSet;

// pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
//     let length = factors.len();
//     if length == 0 {
//         return 0;
//     }

//     let mut visited_nums: HashSet<u32> = HashSet::new();
//     let mut sum = 0;
//     for num in factors {
//         if num == &0 {
//             continue;
//         }
//         let mut n = *num;
//         while n < limit {
//             if !visited_nums.contains(&n) {
//                 visited_nums.insert(n);
//                 sum += n;
//             }
//             n += num;
//         }
//     }
//     sum
// }
pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    (1..limit)
        .filter(|i| factors.iter().any(|f| f != &0 && i % f == 0))
        .sum()
}

#[bench]
fn bench_xor_1000_ints(b: &mut Bencher) {
    b.iter(|| {
        sum_of_multiples(1_000_000_000, &[1, 3, 5, 7, 11]);
    });
}

// #![feature(test)]

// extern crate test;

// pub fn add_two(a: i32) -> i32 {
//     a + 2
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use test::Bencher;

//     #[test]
//     fn it_works() {
//         assert_eq!(4, add_two(2));
//     }

//     #[bench]
//     fn bench_add_two(b: &mut Bencher) {
//         b.iter(|| add_two(2));
//     }
// }

// #![feature(test)]

// extern crate test;
// use test::Bencher;

// #[bench]
// fn bench_xor_1000_ints(b: &mut Bencher) {
//     b.iter(|| {
//         (0..1000).fold(0, |old, new| old ^ new);
//     });
// }
