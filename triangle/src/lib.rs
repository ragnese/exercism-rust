extern crate num;
use num::ToPrimitive;

pub struct Triangle {
    a: f32,
    b: f32,
    c: f32,
}

impl Triangle {
    pub fn build<T: ToPrimitive>(sides: [T; 3]) -> Result<Triangle, ()> {
        let a = sides[0].to_f32().ok_or(())?;
        let b = sides[1].to_f32().ok_or(())?;
        let c = sides[2].to_f32().ok_or(())?;

        if a == 0.0 || b == 0.0 || c == 0.0 {
            return Err(());
        }

        if a + b < c || a + c < b || b + c < a {
            return Err(());
        }

        Ok(Triangle { a: a, b: b, c: c })
    }

    pub fn is_equilateral(&self) -> bool {
        self.a == self.b && self.b == self.c
    }

    pub fn is_isosceles(&self) -> bool {
        (self.a == self.b && self.a != self.c) ||
        (self.a == self.c && self.b != self.c) ||
        (self.b == self.c && self.a != self.b)
    }

    pub fn is_scalene(&self) -> bool {
        !self.is_equilateral() && !self.is_isosceles()
    }

    pub fn is_degenerate(&self) -> bool {
        (self.a + self.b == self.c) ||
        (self.a + self.c == self.b) ||
        (self.b + self.c == self.a)
    }
}
