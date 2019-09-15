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

/// A duration of a note.
/// - Tuplet Numerator
/// - Tuplet Denomerator
/// - Augmentation
#[repr(u8)]
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
            Den64(n, d, a) => Den64(n, d, 1),
            Den32(n, d, a) => Den32(n, d, (a + 1).min(2)),
            Den16(n, d, a) => Den16(n, d, (a + 1).min(3)),
            Den8(n, d, a) => Den8(n, d, (a + 1).min(4)),
            Den4(n, d, a) => Den4(n, d, (a + 1).min(4)),
            Den2(n, d, a) => Den2(n, d, (a + 1).min(3)),
            Num1(n, d, a) => Num1(n, d, (a + 1).min(2)),
            Num2(n, d, a) => Num2(n, d, 1),
            Num4(n, d) => Num4(n, d),
        }
    }

    /// Diminish duration
    pub fn diminish(&mut self) {
        use Duration::*;

        *self = match *self {
            Den128(n, d) => Den128(n, d),
            Den64(n, d, a) => Den64(n, d, 0),
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
}
