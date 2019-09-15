use std::fmt;

/// A Pitch Name.
#[derive(Copy, Clone)]
pub enum PitchName {
    C,
    D,
    E,
    F,
    G,
    A,
    B,
}

/// A Pitch Accidental.
#[derive(Copy, Clone)]
pub enum PitchAccidental {
    ///
    DoubleFlat,
    ///
    FlatQuarterFlat,
    ///
    Flat,
    ///
    QuarterFlat,
    ///
    Natural,
    ///
    QuarterSharp,
    ///
    Sharp,
    ///
    SharpQuarterSharp,
    ///
    DoubleSharp,
}

/// A Pitch Class
#[derive(Copy, Clone)]
pub struct PitchClass {
    pub name: PitchName,
    pub accidental: Option<PitchAccidental>,
}

/// A Pitch Octave
#[derive(Copy, Clone)]
#[repr(i8)]
pub enum PitchOctave {
    /// Octave -1
    Octave_ = -1,
    /// Octave 0
    Octave0 = 0,
    /// Octave 1
    Octave1 = 1,
    /// Octave 2
    Octave2 = 2,
    /// Octave 3
    Octave3 = 3,
    /// Octave 4
    Octave4 = 4,
    /// Octave 5
    Octave5 = 5,
    /// Octave 6
    Octave6 = 6,
    /// Octave 7
    Octave7 = 7,
    /// Octave 8
    Octave8 = 8,
    /// Octave 9
    Octave9 = 9,
}

impl PitchOctave {
    /// Calculate a lower octave.
    pub fn lower(self) -> Option<PitchOctave> {
        use PitchOctave::*;

        match self {
            Octave_ => None,
            Octave0 => Some(Octave_),
            Octave1 => Some(Octave0),
            Octave2 => Some(Octave1),
            Octave3 => Some(Octave2),
            Octave4 => Some(Octave3),
            Octave5 => Some(Octave4),
            Octave6 => Some(Octave5),
            Octave7 => Some(Octave6),
            Octave8 => Some(Octave7),
            Octave9 => Some(Octave8),
        }
    }

    /// Calculate a higher octave.
    pub fn raise(self) -> Option<PitchOctave> {
        use PitchOctave::*;

        match self {
            Octave_ => Some(Octave0),
            Octave0 => Some(Octave1),
            Octave1 => Some(Octave2),
            Octave2 => Some(Octave3),
            Octave3 => Some(Octave4),
            Octave4 => Some(Octave5),
            Octave5 => Some(Octave6),
            Octave6 => Some(Octave7),
            Octave7 => Some(Octave8),
            Octave8 => Some(Octave9),
            Octave9 => None,
        }
    }
}

impl fmt::Display for PitchOctave {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        use PitchOctave::*;

        match self {
            Octave_ => write!(f, "-"),
            Octave0 => write!(f, "0"),
            Octave1 => write!(f, "1"),
            Octave2 => write!(f, "2"),
            Octave3 => write!(f, "3"),
            Octave4 => write!(f, "4"),
            Octave5 => write!(f, "5"),
            Octave6 => write!(f, "6"),
            Octave7 => write!(f, "7"),
            Octave8 => write!(f, "8"),
            Octave9 => write!(f, "9"),
        }
    }
}
