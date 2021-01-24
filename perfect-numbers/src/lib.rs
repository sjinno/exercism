#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

#[rustfmt::skip]
pub fn classify(num: u64) -> Option<Classification> {
    use std::cmp::Ordering::*;

    match num {
        0 => None,
        1 => Some(Classification::Deficient),
        _ => {
            let total;
            if num % 2 == 0 {
                total = (1..=num / 2)
                    .filter(|x| num.rem_euclid(*x) == 0)
                    .sum::<u64>();
            } else {
                total = (1..=num / 3)
                    .filter(|x| num.rem_euclid(*x) == 0)
                    .sum::<u64>();
            }

            match total.cmp(&num) {
                Greater => return Some(Classification::Abundant),
                Equal   => return Some(Classification::Perfect),
                Less    => return Some(Classification::Deficient),
            }
        }
    }
}
