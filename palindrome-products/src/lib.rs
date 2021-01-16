#[derive(Debug, PartialEq, Eq)]
pub struct Palindrome {
    smallest: Vec<(usize, usize)>,
    largest: Vec<(usize, usize)>,
}

impl Palindrome {
    pub fn new(a: u64, b: u64) -> Palindrome {
        unimplemented!("create a palindrome with factors ({}, {})", a, b)
    }

    pub fn value(&self) -> u64 {
        unimplemented!("return the value of this palindrome")
    }

    pub fn insert(&mut self, a: u64, b: u64) {
        unimplemented!("insert new factors ({}, {}) into this palindrome", a, b)
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let smallest = std::f64::INFINITY;
    let mut smallest_factors = Vec::<(u64, u64)>::new();
    for a in min..=max {
        for b in min..=max {
            let ab = a as f64 * b as f64;
            let pal = ab.clone().to_string().chars().collect::<Vec<char>>();
            let reversed = pal.clone().into_iter().rev().collect::<Vec<char>>();
            if pal == reversed {
                if ab < smallest {
                    smallest_factors.push((a, b));
                    break;
                }
            }
        }
        if !smallest_factors.is_empty() {
            break;
        }
    }

    let mut largest = 0;
    let mut largest_factors = vec![];
    for a in (min..=max).rev() {
        for b in (min..=max).rev() {
            let ab = a * b;
            let pal = ab.to_string().chars().collect::<Vec<char>>();
            let reversed = pal.clone().into_iter().rev().collect::<Vec<char>>();
            if pal == reversed {
                if ab > largest {
                    largest = ab;
                    largest_factors.clear();
                    largest_factors.push((a, b));
                } else if ab == largest {
                    largest_factors.push((a, b));
                }
            }
            if largest_factors.len() == 2 {
                break;
            }
        }
        if largest_factors.len() == 2 {
            break;
        }
    }

    None
}
