#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

fn check_validity(number: &[u32], from_base: &u32, to_base: &u32) -> Result<(), Error> {
    match (from_base, to_base) {
        (0 | 1, _) => Err(Error::InvalidInputBase),
        (_, 0 | 1) => Err(Error::InvalidOutputBase),
        (_, _) => {
            if let Some(num) = number.iter().find(|elt| *elt >= from_base) {
                Err(Error::InvalidDigit(*num))
            } else {
                Ok(())
            }
        }
    }
}

pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    check_validity(number, &from_base, &to_base).map(|_| {
        if number.is_empty() {
            return vec![0];
        }

        use std::collections::VecDeque;

        let mut ans = VecDeque::new();

        let mut whole_number = {
            let max_pow = number.len() - 1;
            number.iter().enumerate().fold(0, |acc, (idx, elt)| {
                acc + elt * from_base.pow((max_pow - idx) as u32)
            })
        };

        loop {
            let rem = whole_number.rem_euclid(to_base);
            ans.push_front(rem);

            whole_number = whole_number.div_euclid(to_base);
            if whole_number == 0 {
                break;
            }
        }

        ans.into()
    })
}
