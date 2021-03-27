use num::Zero; // num = "0.4"
use std::ops::Add;

pub struct Triangle<T> {
    sides: [T; 3],
}

impl<T> Triangle<T>
where
    T: Add<Output = T> + Copy + PartialOrd + Zero,
{
    pub fn build(sides: [T; 3]) -> Option<Triangle<T>> {
        if sides.iter().any(|n| n.is_zero()) {
            return None;
        }
        let triangle = Triangle { sides };
        let (a, b, c) = (sides[0], sides[1], sides[2]);
        if a + b >= c && b + c >= a && c + a >= b {
            Some(triangle)
        } else {
            None
        }
    }

    pub fn is_equilateral(&self) -> bool {
        self.sides[0] == self.sides[1] && self.sides[1] == self.sides[2]
    }

    pub fn is_scalene(&self) -> bool {
        !self.is_equilateral() && !self.is_isosceles()
    }

    pub fn is_isosceles(&self) -> bool {
        self.sides[0] == self.sides[1]
            || self.sides[0] == self.sides[2]
            || self.sides[1] == self.sides[2]
    }
}
