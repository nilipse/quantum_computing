use std::fmt;
use std::ops::{Add, Mul};

#[derive(Debug, Clone, Copy)]
struct Complex(i32, i32);

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + {}i", self.0, self.1)
    }
}

impl Add for Complex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Mul for Complex {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Complex(
            self.0 * rhs.0 - self.1 * rhs.1,
            self.0 * rhs.1 + self.1 * rhs.0,
        )
    }
}

fn main() {
    let x = Complex(0, 1);
    let y = Complex(0, 1);
    println!("{}", x * y);
    println!("{}", x + y);
}
