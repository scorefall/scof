//! Fraction

use std::ops::{Mul, Add, Sub, Div};
use std::convert::TryInto;
use std::cmp::Ordering;

/// (Unsigned) Fraction of a measure.
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Fraction {
    pub num: u8,
    pub den: u8,
}

impl Fraction {
    /// Create a new fraction of a measure from a tuple.
    pub fn new(num: u8, den: u8) -> Self {
        Self { num, den }
    }

    /// Reciprocal (1 / self).
    pub fn recip(self) -> Self {
        Self { num: self.den, den: self.num }
    }
}

impl Mul<i32> for Fraction {
    type Output = i32;

    fn mul(self, other: i32) -> Self::Output {
        let num = f32::from(self.num);
        let den = f32::from(self.den);
        (other as f32 * num * den.recip()) as i32
    }
}

impl Mul for Fraction {
    type Output = Fraction;

    fn mul(self, other: Fraction) -> Self::Output {
        let mut num: u16 = self.num.into();
        let mut den: u16 = self.den.into();
        let other_num: u16 = other.num.into();
        let other_den: u16 = other.den.into();

        num *= other_num;
        den *= other_den;

        let gcd = gcd_i(num, den);

        Fraction {
            num: (num / gcd).try_into().unwrap_or(0),
            den: (den / gcd).try_into().unwrap_or(0),
        }
    }
}

impl Div for Fraction {
    type Output = Fraction;

    fn div(self, other: Fraction) -> Self::Output {
        self * other.recip()
    }
}

impl Add for Fraction {
    type Output = Fraction;

    fn add(self, other: Fraction) -> Self::Output {
        let (self_mul, other_mul, den) = if self.den % other.den == 0 {
            (1, self.den / other.den, self.den)
        } else if other.den % self.den == 0 {
            (other.den / self.den, 1, other.den)
        } else {
            (other.den, self.den, self.den * other.den)
        };

        let num = self.num * self_mul + other.num * other_mul;
        let gcd = gcd_i(num, den);
        Fraction {
            num: num / gcd,
            den: den / gcd,
        }
    }
}

impl Sub for Fraction {
    type Output = Fraction;

    fn sub(self, other: Fraction) -> Self::Output {
        let (self_mul, other_mul, den) = if self.den % other.den == 0 {
            (1, self.den / other.den, self.den)
        } else if other.den % self.den == 0 {
            (other.den / self.den, 1, other.den)
        } else {
            (other.den, self.den, self.den * other.den)
        };

        let num = self.num * self_mul - other.num * other_mul;
        let gcd = gcd_i(num, den);
        Fraction {
            num: num / gcd,
            den: den / gcd,
        }
    }
}

impl PartialOrd for Fraction {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let den = gcd_i(self.den, other.den);

        let self_mul = (den / self.den) as i32;
        let other_mul = (den / other.den) as i32;

        let num = self.num as i32 * self_mul - other.num as i32 * other_mul;

        num.partial_cmp(&0)
    }
}

pub trait IsZero {
    fn is_zero(self) -> bool;
}

impl IsZero for u8 {
    fn is_zero(self) -> bool {
        self == 0
    }
}

impl IsZero for u16 {
    fn is_zero(self) -> bool {
        self == 0
    }
}


impl IsZero for u32 {
    fn is_zero(self) -> bool {
        self == 0
    }
}


impl IsZero for u64 {
    fn is_zero(self) -> bool {
        self == 0
    }
}


impl IsZero for u128 {
    fn is_zero(self) -> bool {
        self == 0
    }
}

impl IsZero for Fraction {
    fn is_zero(self) -> bool {
        self.num == 0 && self.den != 0
    }
}

// Iterative Greatest Common Divisor.
fn gcd_i<T>(mut a: T, mut b: T) -> T
    where T: PartialEq + std::ops::RemAssign + IsZero + Copy + Clone
{
    if a.is_zero() {
        return b;
    } else if b.is_zero() {
        return a;
    }

    loop {
        a %= b;
        if a.is_zero() {
            return b;
        }
        b %= a;
        if b.is_zero() {
            return a;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        assert_eq!(Fraction::new(1, 2) + Fraction::new(3, 4), Fraction::new(5, 4));
        assert_eq!(Fraction::new(1, 8) + Fraction::new(1, 2), Fraction::new(5, 8));
        assert_eq!(Fraction::new(1, 1) + Fraction::new(10, 1), Fraction::new(11, 1));
        assert_eq!(Fraction::new(1, 3) + Fraction::new(1, 5), Fraction::new(8, 15));
        assert_eq!(Fraction::new(4, 4) + Fraction::new(2, 4), Fraction::new(3, 2));
    }

    #[test]
    fn sub() {
        assert_eq!(Fraction::new(5, 4) - Fraction::new(1, 2), Fraction::new(3, 4));
        assert_eq!(Fraction::new(1, 1) - Fraction::new(1, 64), Fraction::new(63, 64));
    }

    #[test]
    fn div() {
        assert_eq!(Fraction::new(1, 2) / Fraction::new(3, 4), Fraction::new(2, 3));
    }

    #[test]
    fn mul() {
        assert_eq!(Fraction::new(1, 2) * Fraction::new(3, 4), Fraction::new(3, 8));
    }
}
