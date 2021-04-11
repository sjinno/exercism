use std::fmt;

use luhn_from::Luhn as FromLuhn;

pub trait Luhn {
    fn valid_luhn(&self) -> bool;
}

impl<T: fmt::Display> Luhn for T {
    fn valid_luhn(&self) -> bool {
        FromLuhn::from(self).0
    }
}
