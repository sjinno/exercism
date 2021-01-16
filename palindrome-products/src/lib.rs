#[derive(Debug, PartialEq, Eq)]
pub struct Palindrome {
    val: u64,
    factors: Vec<(u64, u64)>,
}

impl Palindrome {
    pub fn new(a: u64, b: u64) -> Palindrome {
        Self {
            val: a * b,
            factors: vec![(a, b)],
        }
    }

    pub fn value(&self) -> u64 {
        self.val
    }

    pub fn insert(&mut self, a: u64, b: u64) {
        self.factors.push((a, b))
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let mut sm = Palindrome::new(max, max);
    let mut sm_found = false;
    let mut lg = Palindrome::new(0, 0);

    for a in min..=max {
        for b in min..=max {
            let ab = a * b;
            let p = ab.to_string();
            let reversed = p.clone().chars().rev().collect::<String>();
            if p == reversed {
                sm.factors = vec![(b, a)];
                sm.val = ab;
                sm_found = true;
                break;
            }
        }
        if sm_found {
            break;
        }
    }

    let mut largest = 0;
    let mut count = 0;
    for a in (min..=max).rev() {
        for b in (min..=max).rev() {
            let ab = a * b;
            let p = ab.to_string();
            let reversed = p.clone().chars().rev().collect::<String>();
            if p == reversed {
                if ab > largest {
                    largest = ab;
                    lg.factors = vec![(b, a)];
                    lg.val = ab;
                    count += 1;
                } else if ab == largest {
                    lg.factors.push((b, a));
                    count += 1;
                }
            }
            if count == 2 {
                break;
            }
        }
        if count == 2 {
            break;
        }
    }

    if lg.factors.len() == 2 {
        if lg.factors[0].0 == lg.factors[1].1 {
            lg.factors.remove(1);
        }
    }

    if !sm_found {
        return None;
    }
    Some((sm, lg))
}
