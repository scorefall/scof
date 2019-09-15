//! # Note (convert Note Struct <-> String)
//! A note has an optional pitch, None = Rest.
//! 
//! ## Structure
//! 
//! **note length**: Required number for note duration 8=eighth note.
//! 
//! - O: 128th note
//! - X: 64th note
//! - Y: 32nd note
//! - S: 16th note
//! - T: 8th note
//! - Q: quarter note
//! - U: half note
//! - W: whole note
//! - V: double whole note (breve)
//! - L: quadruple whole note (longa)
//! - .: augmentation dot
//! 
//! **note name**: Required name of the note.  A-G, or R for rest.
//! 
//! - `A`
//! - `B`
//! - `C`
//! - `D`
//! - `E`
//! - `F`
//! - `G`
//! - `R`
//! 
//! **accidental**: Optional accidental.  If not provided, from key signature.  Cannot be same as what is in the key signature.
//! 
//! - `bb`: Double Flat (Whole-Tone Flat)
//! - `db`: 3/4-Tone Flat
//! - `b`: Flat (1/2-Tone Flat)
//! - `d`: 1/4-Tone Flat
//! - `n`: Natural
//! - `t`: 1/4-Tone Sharp
//! - `#`: Sharp (1/2-Tone Sharp)
//! - `t#`: 3/4-Tone Sharp
//! - `x`: Double Sharp (Whole-Tone Sharp)
//! 
//! **octave**: Required octave.  `-`=-1,`0`,`1`,`2`,`3`,`4`,`5`,`6`,`7`,`8`,`9`
//! 
//! **articulation**: Optional articulation.
//! 
//! - `^`: Marcato (separated sharp attack)
//! - `>`: Accent (sharp attack)
//! - `.`: Staccato (separated)
//! - `'`: Staccatissimo (very separated)
//! - `_`: Tenuto (connected)
//! - `_.`: Tenuto Staccato
//! - `^.`: Marcato Staccato
//! - `^_`: Marcato Tenuto
//! - `>.`: Accent Staccato
//! - `>_`: Accent Tenuto

use std::{fmt, str::FromStr};
use crate::Fraction;

mod articulation;
mod pitch;
mod duration;

pub use self::articulation::*;
pub use self::pitch::*;
pub use self::duration::*;

/// A note.
pub struct Note {
    /// Pitch & Octave
    pub pitch: Option<(PitchClass, PitchOctave)>,
    /// Duration of the note as a fraction.
    pub duration: Fraction,
    /// Articulation.
    pub articulation: Vec<Articulation>,
}

impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write duration.
        if self.duration.num != 1 {
            write!(f, "{}/", self.duration.num)?;
        }
        write!(f, "{}", self.duration.den)?;

        // Write note name & octave.
        match &self.pitch {
            Some(pitch) => {
                let class = match pitch.0.name {
                    PitchName::A => "A",
                    PitchName::B => "B",
                    PitchName::C => "C",
                    PitchName::D => "D",
                    PitchName::E => "E",
                    PitchName::F => "F",
                    PitchName::G => "G",
                };
                write!(f, "{}{}", class, pitch.1)
            },
            None => write!(f, "R"),
        }
    }
}

impl FromStr for Note {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Read duration.
        let start_index = 0;
        let mut end_index = 0;
        for (i, c) in s.char_indices() {
            if !c.is_numeric() {
                end_index = i;
                break;
            }
        }

        // Read rest of duration.
        let duration = if s[end_index..].chars().next() == Some('/') {
            let numer = s[start_index..end_index].parse::<u8>().or(Err(()))?;

            end_index += 1;

            let start_index = end_index;

            for (i, c) in s[start_index..].char_indices() {
                if !c.is_numeric() {
                    end_index = i;
                    break;
                }
            }

            let denom = s[start_index..end_index].parse::<u8>().or(Err(()))?;

            Fraction::new(numer, denom)
        } else {
            let numer = 1;
            let denom = s[start_index..end_index].parse::<u8>().or(Err(()))?;

            Fraction::new(numer, denom)
        };
        let articulation = vec![];

        // Read note name.
        match s.get(end_index..).ok_or(())? {
            "R" => Ok(Note {
                pitch: None,
                duration,
                articulation,
            }),
            a => {
                let two = a.chars().collect::<Vec<char>>();
                let letter_name = two[0];
                let octave_num = match two[1] {
                    '-' => PitchOctave::Octave_,
                    '0' => PitchOctave::Octave0,
                    '1' => PitchOctave::Octave1,
                    '2' => PitchOctave::Octave2,
                    '3' => PitchOctave::Octave3,
                    '4' => PitchOctave::Octave4,
                    '5' => PitchOctave::Octave5,
                    '6' => PitchOctave::Octave6,
                    '7' => PitchOctave::Octave7,
                    '8' => PitchOctave::Octave8,
                    '9' => PitchOctave::Octave9,
                    _ => return Err(()),
                };

                Ok(Note {
                    pitch: Some((
                        match letter_name {
                            'A' => PitchClass {
                                name: PitchName::A,
                                accidental: None,
                            },
                            'B' => PitchClass {
                                name: PitchName::B,
                                accidental: None,
                            },
                            'C' => PitchClass {
                                name: PitchName::C,
                                accidental: None,
                            },
                            'D' => PitchClass {
                                name: PitchName::D,
                                accidental: None,
                            },
                            'E' => PitchClass {
                                name: PitchName::E,
                                accidental: None,
                            },
                            'F' => PitchClass {
                                name: PitchName::F,
                                accidental: None,
                            },
                            'G' => PitchClass {
                                name: PitchName::G,
                                accidental: None,
                            },
                            // FIXME: return Err
                            a => panic!("Failed to parse '{}'", a),
                        },
                        octave_num,
                    )),
                    duration,
                    articulation,
                })
            }
        }
    }
}

impl Note {
    /// Get the note's visual distance from middle C (C4).
    pub fn visual_distance(&self) -> i32 {
        if let Some(ref pitch) = self.pitch {
            // Calculate number of octaves from middle C (C4).
            let octaves = pitch.1 as i32 - 4;
            // Calculate number of steps from C within key.
            let steps = match pitch.0.name {
                PitchName::C => 0,
                PitchName::D => 1,
                PitchName::E => 2,
                PitchName::F => 3,
                PitchName::G => 4,
                PitchName::A => 5,
                PitchName::B => 6,
            };
            // Calculate total number of steps from middle C.
            let total_steps = steps + octaves * 7;

            // Invert value, because higher notes = -y
            -total_steps
        } else {
            0
        }
    }

    /// Set pitch class and octave.
    pub fn set_pitch(&mut self, pitch: (PitchClass, PitchOctave)) {
        self.pitch = Some(pitch);
    }

    /// Set duration of note.
    pub fn set_duration(&mut self, duration: Fraction) {
        self.duration = duration;
    }

    fn move_step(&self, create: (PitchClass, PitchOctave), run: &dyn Fn(&(PitchClass, PitchOctave)) -> Option<(PitchClass, PitchOctave)>) -> Note {
        let pitch = if let Some(ref pitch) = self.pitch {
            (run)(pitch)
        } else {
            Some(create)
        };

        Note {
            pitch,
            duration: self.duration,
            articulation: self.articulation.clone(),
        }
    }

    /// Calculate note one quarter step up.
    pub fn quarter_step_up(&self, create: (PitchClass, PitchOctave)) -> Note {
        self.step_up(create) // FIXME
    }

    /// Calculate note one quarter step down.
    pub fn quarter_step_down(&self, create: (PitchClass, PitchOctave)) -> Note {
        self.step_down(create) // FIXME
    }

    /// Calculate note one half step up.
    pub fn half_step_up(&self, create: (PitchClass, PitchOctave)) -> Note {
        self.step_up(create) // FIXME
    }

    /// Calculate note one half step down.
    pub fn half_step_down(&self, create: (PitchClass, PitchOctave)) -> Note {
        self.step_down(create) // FIXME
    }

    /// Calculate note one step up within the key.
    /// - `create`: Note that is generated from a rest.
    pub fn step_up(&self, create: (PitchClass, PitchOctave)) -> Note {
        self.move_step(create, &|pitch| {
            let (pitch_class, offset) = match pitch.0.name {
                PitchName::A => (PitchName::B, false),
                PitchName::B => (PitchName::C, true),
                PitchName::C => (PitchName::D, false),
                PitchName::D => (PitchName::E, false),
                PitchName::E => (PitchName::F, false),
                PitchName::F => (PitchName::G, false),
                PitchName::G => (PitchName::A, false),
            };
            let pitch_octave = if offset {
                pitch.1.raise()
            } else {
                Some(pitch.1)
            };

            if let Some(pitch_octave) = pitch_octave {
                Some((PitchClass {
                    name: pitch_class,
                    accidental: pitch.0.accidental,
                }, pitch_octave))
            } else {
                Some((pitch.0, pitch.1))
            }
        })
    }

    /// Calculate note one step down within the key.
    /// - `create`: Note that is generated from a rest.
    pub fn step_down(&self, create: (PitchClass, PitchOctave)) -> Note {
        self.move_step(create, &|pitch| {
            let (pitch_class, offset) = match pitch.0.name {
                PitchName::A => (PitchName::G, false),
                PitchName::B => (PitchName::A, false),
                PitchName::C => (PitchName::B, true),
                PitchName::D => (PitchName::C, false),
                PitchName::E => (PitchName::D, false),
                PitchName::F => (PitchName::E, false),
                PitchName::G => (PitchName::F, false),
            };
            let pitch_octave = if offset {
                pitch.1.lower()
            } else {
                Some(pitch.1)
            };

            if let Some(pitch_octave) = pitch_octave {
                Some((PitchClass {
                    name: pitch_class,
                    accidental: pitch.0.accidental,
                }, pitch_octave))
            } else {
                Some((pitch.0, pitch.1))
            }
        })
    }
}
