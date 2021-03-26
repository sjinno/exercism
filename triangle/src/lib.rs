pub struct Triangle {
    a: u64,
    b: u64,
    c: u64,
}

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        let triangle = Triangle {
            a: sides[0],
            b: sides[1],
            c: sides[2],
        };

        match triangle.is_triangle() {
            true => Some(triangle),
            false => None,
        }
    }

    pub fn is_equilateral(&self) -> bool {
        self.a == self.b && self.b == self.c
    }

    pub fn is_scalene(&self) -> bool {
        !self.is_equilateral() && !self.is_isosceles()
    }

    pub fn is_isosceles(&self) -> bool {
        self.a == self.b || self.a == self.c || self.b == self.c
    }

    fn is_triangle(&self) -> bool {
        if self.a == 0 || self.b == 0 || self.c == 0 {
            false
        } else {
            check_triangularity(&self.a, &self.b, &self.c)
                && check_triangularity(&self.a, &self.c, &self.b)
                && check_triangularity(&self.b, &self.c, &self.a)
        }
    }
}

fn check_triangularity(a: &u64, b: &u64, c: &u64) -> bool {
    let two_sides = a + b;
    two_sides >= *c
}
