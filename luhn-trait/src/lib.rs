use std::fmt;

use luhn_from::Luhn as LuhnFrom;

pub trait Luhn {
    fn valid_luhn(&self) -> bool;
}

impl<T: fmt::Display> Luhn for T {
    fn valid_luhn(&self) -> bool {
        LuhnFrom::from(self).0
    }
}
