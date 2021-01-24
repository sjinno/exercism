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
            let mut divisor = 2;
            while num % divisor != 0 {
                divisor += 1;
            }

            let total = (1..=num / divisor)
                .filter(|x| num % *x == 0)
                .sum::<u64>();

            match total.cmp(&num) {
                Greater => return Some(Classification::Abundant),
                Equal   => return Some(Classification::Perfect),
                Less    => return Some(Classification::Deficient),
            }
        }
    }
}
