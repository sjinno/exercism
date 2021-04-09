use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result};
use std::iter;

use lazy_static::lazy_static;

#[rustfmt::skip]
lazy_static! {
    static ref RN: HashMap::<u32, char> = {
        let mut rn = HashMap::new();
        rn.insert(   1, 'I');
        rn.insert(   5, 'V');
        rn.insert(  10, 'X');
        rn.insert(  50, 'L');
        rn.insert( 100, 'C');
        rn.insert( 500, 'D');
        rn.insert(1000, 'M');
        rn
    };
}

pub struct Roman(String);

impl Roman {
    fn get_roman_numeral(d: u32, num: &u32) -> String {
        let mut rn = String::new();
        match num {
            (1..=3) => (0..*num).for_each(|_| rn.push(*RN.get(&d).unwrap())),
            4 => {
                rn.push(*RN.get(&d).unwrap());
                rn.push(*RN.get(&(d * 5)).unwrap());
            }
            5 => rn.push(*RN.get(&(d * 5)).unwrap()),
            (6..=8) => {
                rn.push(*RN.get(&(d * 5)).unwrap());
                (0..*num - 5).for_each(|_| rn.push(*RN.get(&d).unwrap()));
            }
            9 => {
                rn.push(*RN.get(&d).unwrap());
                rn.push(*RN.get(&(d * 10)).unwrap());
            }
            _ => (),
        }
        rn
    }
}

impl Display for Roman {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.0.fmt(f)
    }
}

impl From<u32> for Roman {
    fn from(num: u32) -> Self {
        let mut num = num;
        let mut divisor = 1000;

        let to_iter = iter::from_fn(move || {
            let quotient = num / divisor;
            num -= quotient * divisor;
            divisor /= 10;
            Some(quotient)
        })
        .take(4)
        .collect::<Vec<_>>();

        Self(
            to_iter
                .iter()
                .enumerate()
                .map(|(i, n)| match i {
                    0 => Self::get_roman_numeral(1000, n),
                    1 => Self::get_roman_numeral(100, n),
                    2 => Self::get_roman_numeral(10, n),
                    3 => Self::get_roman_numeral(1, n),
                    _ => unreachable!(),
                })
                .collect::<String>(),
        )
    }
}
