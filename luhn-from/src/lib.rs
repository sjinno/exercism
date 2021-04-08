use std::fmt;

use luhn;

pub struct Luhn(pub bool);

impl Luhn {
    pub fn is_valid(&self) -> bool {
        self.0
    }
}

impl<T: fmt::Display> From<T> for Luhn {
    fn from(input: T) -> Self {
        Self(luhn::is_valid(&input.to_string()))
    }
}
