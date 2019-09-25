//! # Tuplets
//! Tuplets have a numerator and a denomerator.  If the denomerator is a power
//! of two and the tuplet lands on a power of 2 beat fraction, then the
//! denomerator doesn't have to be rendered.
//!
//! 3:2 is three half notes where there would normally be 2.  The numerator
//! should always be more than the denomerator, but 5:2 should not be allowed.
//! 5:2 is 5 notes where there would normally be 2, better written as 5:4 (at
//! least in 4/4).
//!
//! For a 5/4 time signature, write 6:5 for a sextuplet.
//! For no tuple, 1:1.

use std::{fmt, str::FromStr};
use crate::Fraction;

/// A duration of a note.
/// - Tuplet Numerator
/// - Tuplet Denomerator
/// - Augmentation
#[repr(u8)]
#[derive(Copy, Clone)]
pub enum Duration {
    /// No augmentation allowed
    Den128(u8, u8),
    /// 0 or 1 augmentation dots
    Den64(u8, u8, u8),
    /// 0, 1 or 2 augmentation dots
    Den32(u8, u8, u8),
    /// 0, 1, 2 or 3 augmentation dots
    Den16(u8, u8, u8),
    /// 0, 1, 2, 3 or 4 augmentation dots
    Den8(u8, u8, u8),
    /// 0, 1, 2, 3 or 4 augmentation dots
    Den4(u8, u8, u8),
    /// 0, 1, 2 or 3 augmentation dots
    Den2(u8, u8, u8),
    /// 0, 1 or 2 augmentation dots
    Num1(u8, u8, u8),
    /// 0 or 1 augmentation dots
    Num2(u8, u8, u8),
    /// No augmentation allowed
    Num4(u8, u8),
}

impl Duration {
    /// Augment duration
    pub fn augment(&mut self) {
        use Duration::*;

        *self = match *self {
            Den128(n, d) => Den128(n, d),
            Den64(n, d, _a) => Den64(n, d, 1),
            Den32(n, d, a) => Den32(n, d, (a + 1).min(2)),
            Den16(n, d, a) => Den16(n, d, (a + 1).min(3)),
            Den8(n, d, a) => Den8(n, d, (a + 1).min(4)),
            Den4(n, d, a) => Den4(n, d, (a + 1).min(4)),
            Den2(n, d, a) => Den2(n, d, (a + 1).min(3)),
            Num1(n, d, a) => Num1(n, d, (a + 1).min(2)),
            Num2(n, d, _a) => Num2(n, d, 1),
            Num4(n, d) => Num4(n, d),
        }
    }

    /// Diminish duration
    pub fn diminish(&mut self) {
        use Duration::*;

        *self = match *self {
            Den128(n, d) => Den128(n, d),
            Den64(n, d, _a) => Den64(n, d, 0),
            Den32(n, d, a) => Den32(n, d, a.saturating_sub(1)),
            Den16(n, d, a) => Den16(n, d, a.saturating_sub(1)),
            Den8(n, d, a) => Den8(n, d, a.saturating_sub(1)),
            Den4(n, d, a) => Den4(n, d, a.saturating_sub(1)),
            Den2(n, d, a) => Den2(n, d, a.saturating_sub(1)),
            Num1(n, d, a) => Num1(n, d, a.saturating_sub(1)),
            Num2(n, d, a) => Num2(n, d, 0),
            Num4(n, d) => Num4(n, d),
        }
    }

    /// Convert duration into fraction of a quarter note.
    pub fn fraction(&self) -> Fraction {
        use Duration::*;

        match *self {
            Den128(n, d) => Fraction::new(1, 128) * Fraction::new(d, n),
            Den64(n, d, _a) => Fraction::new(1, 64) * Fraction::new(d, n),
            Den32(n, d, _a) => Fraction::new(1, 32) * Fraction::new(d, n),
            Den16(n, d, _a) => Fraction::new(1, 16) * Fraction::new(d, n),
            Den8(n, d, _a) => Fraction::new(1, 8) * Fraction::new(d, n),
            Den4(n, d, _a) => Fraction::new(1, 4) * Fraction::new(d, n),
            Den2(n, d, _a) => Fraction::new(1, 2) * Fraction::new(d, n),
            Num1(n, d, _a) => Fraction::new(1, 1) * Fraction::new(d, n),
            Num2(n, d, _a) => Fraction::new(2, 1) * Fraction::new(d, n),
            Num4(n, d) => Fraction::new(4, 1) * Fraction::new(d, n),
        }
    }
}

impl fmt::Display for Duration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Duration::*;

        fn augment(f: &mut fmt::Formatter, c: char, a: u8) -> fmt::Result {
            write!(f, "{}", c)?;
            for i in 0..a {
                write!(f, ".")?;
            }
            Ok(())
        }

        match *self {
            Den128(_n, _d) => write!(f, "O"),
            Den64(_n, _d, a) => augment(f, 'X', a),
            Den32(_n, _d, a) => augment(f, 'Y', a),
            Den16(_n, _d, a) => augment(f, 'S', a),
            Den8(_n, _d, a) => augment(f, 'T', a),
            Den4(_n, _d, a) => augment(f, 'Q', a),
            Den2(_n, _d, a) => augment(f, 'U', a),
            Num1(_n, _d, a) => augment(f, 'W', a),
            Num2(_n, _d, a) => augment(f, 'V', a),
            Num4(_n, _d) => write!(f, "L"),
        }
    }
}

pub(super) struct Duration2(pub(super) Vec<Duration>);

impl FromStr for Duration2 {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut end_index = 0;
        let mut iter = s.chars();
        let mut out = vec![];

        for c in iter {
            match c {
                'O' => out.push(Duration::Den128(1, 1)),
                'X' => out.push(Duration::Den64(1, 1, 0)),
                'Y' => out.push(Duration::Den32(1, 1, 0)),
                'S' => out.push(Duration::Den16(1, 1, 0)),
                'T' => out.push(Duration::Den8(1, 1, 0)),
                'Q' => out.push(Duration::Den4(1, 1, 0)),
                'U' => out.push(Duration::Den2(1, 1, 0)),
                'W' => out.push(Duration::Num1(1, 1, 0)),
                'V' => out.push(Duration::Num2(1, 1, 0)),
                'L' => out.push(Duration::Num4(1, 1)),
                '.' => {
                    if let Some(mut dur) = out.pop() {
                        dur.augment(); // TODO: make fail, if can't augment.
                        out.push(dur);
                    } else {
                        return Err(());
                    }
                }
                _ => return Err(())
            }
        };

        Ok(Duration2(out))
    }
}
