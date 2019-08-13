// Scof - A Music Score File Format
//
// Copyright (C) 2019 Jeron Aldaron Lau <jeronlau@plopgrizzly.com>, Doug P. Lau
//
//     This program is free software: you can redistribute it and/or modify
//     it under the terms of the GNU General Public License as published by
//     the Free Software Foundation, either version 3 of the License, or
//     (at your option) any later version.
// 
//     This program is distributed in the hope that it will be useful,
//     but WITHOUT ANY WARRANTY; without even the implied warranty of
//     MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//     GNU General Public License for more details.
// 
//     You should have received a copy of the GNU General Public License
//     along with this program.  If not, see <https://www.gnu.org/licenses/>.

use muon_rs as muon;
use serde_derive::{Deserialize, Serialize};

/// A Pitch Name.
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
pub struct PitchClass {
    pub name: PitchName,
    pub accidental: Option<PitchAccidental>,
}

/// A Dynamic.
pub enum Dynamic {
    PPPPPP,
    PPPPP,
    PPPP,
    PPP,
    PP,
    P,
    MP,
    MF,
    F,
    FF,
    FFF,
    FFFF,
    FFFFF,
    FFFFFF,
    N,
    SF, SFZ,
    FP, SFP,
}

/// An articulation (affects how the note is played).
pub enum Articulation {
    /// Really separated.
    Staccatissimo,
    /// Separated (short 1/2)
    Staccato,
    /// Tenuto
    Tenuto,
    /// Tenuto Staccato
    TenutoStaccato,
    /// Marcato (short sharp attack) (2/3)
    Marcato,
    /// Marcato Staccato
    MarcatoStaccato,
    /// Marcato Tenuto
    MarcatoTenuto,
    /// Accent (sharp attack)
    Accent,
    /// Accent Staccato
    AccentStaccato,
    /// Accent Tenuto
    AccentTenuto,
    /// Slur
    Slur,
    /// Glissando
    Glissando,
    /// Pitch bend slide up into
    BendUpInto,
    /// Pitch bend slide down into
    BendDownInto,
    /// Pitch bend slide up out of
    BendUpOut,
    /// Pitch bend slide down out of (fall)
    BendDownOut,
    /// Fermata (everyone plays long)
    Fermata,
    /// Closed mute (or palm mute rendered as _ on guitar)
    Mute,
    /// Open (no) Mute
    Open,
    /// Harmonic
    Harmonic,
    /// Turn
    Turn,
    /// Inverted Turn
    TurnInverted,
    /// Trill
    Trill,
    /// Tremelo
    Tremelo,
    /// Arpeggio (strum) pitch up, strum guitar down.
    StrumDown,
    /// Arpeggio (strum) pitch down, strum guitar up
    StrumUp,
    /// Pedal
    Pedal,
}

/// A note.
pub struct Note {
    /// Pitch & Octave
    pub pitch: Option<(PitchClass, i8)>,
    /// Duration of the note as a fraction.
    pub duration: (u8, u8),
    /// Articulation.
    pub articulation: Option<Articulation>,
}

/// A marking.
pub enum Marking {
    /// Change intensity of sound.
    Dynamic(Dynamic),
    /// Grace Note into
    GraceInto(Note),
    /// Grace Note from
    GraceOutOf(Note),
    /// Note
    Note(Note),
    /// Breath
    Breath,
    /// Short grand pause for all instruments
    CaesuraShort,
    /// Long grand pause for all instruments
    CaesuraLong,
    /// Increase intensity
    Cresc,
    /// Decrease intensity
    Dim,
    /// Pizzicato (pluck)
    Pizz,
    /// Arco (bowed)
    Arco,
    /// Standard Mute [con sordino]
    Mute,
    /// Open (no mute) [senza sordino]
    Open,
    /// Repeat
    Repeat,
}

/// A repeat marking for a measure.
pub enum Repeat {
    /// Repeat sign open ||:
    Open,
    /// Repeat sign close :||
    Close,
    /// Sign (to jump backwards to).
    Segno,
    /// Jump back to beginning.
    DC,
    /// Jump back to sign.
    DS,
    /// The marks the beginning of the coda.
    Coda,
    /// Jump forward to the coda.
    ToCoda,
    /// End here (after jumping backwards to the sign).
    Fine,
    /// Numbered ending.
    Ending(u8),
}

/////////////////////
////             ////
/////////////////////

/// A waveform.
pub struct Waveform {
    /// True: Signed 16-bit integer, False: Signed 8-bit integer.
    si16: bool,
    /// True: Waveform doesn't loop, False: Waveform loops.
    once: bool,
    /// Hexadecimal string representation of waveform.
    wave: String,
}

/// Reverb & other effect settings.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Effect {
    // TODO
}

/// Channel definition for synthesis.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SynthChan {
    /// Name of instrument sounds.
    waveform: Vec<String>,
    /// Instrument effects.
    effect: Vec<u32>,
//    ///
//    sounds: Vec<Sound>,
    /// Volume: 0-1
    volume: f32,

}

/// Synthesis file.
#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Synth {
    /// Instrument presets, 
    /// Reverb presets, IDs automatically assigned.
    effect: Vec<Effect>,
    /// Channels
    chan: Vec<SynthChan>,
}

/// A signature.
#[derive(PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct Sig {
    /// The key signature (0-23 quarter steps above C, 24+ reserved for middle
    /// eastern and Indian key signatures).
    pub key: u8,
    /// Time signature (num_beats/note_len), 4/4 is common.
    pub time: String,
    /// BPM (beats per minute), 120 is common (default=120).
    pub tempo: u16,
    /// % Swing (default=50).
    pub swing: Option<u8>,
}

/// Channel information for a specific bar of music.
#[derive(PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct Chan {
    notes: Vec<String>,
    lyric: Option<String>,
}

/// A bar (or measure) of music.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Bar {
    /// Signature index
    pub sig: Option<u32>,
    /// All of the channels in this piece.
    pub chan: Vec<Chan>,
    /// Repeat symbols for this measure.
    pub repeat: Vec<String>,
}

/// A movement in the score.
#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct Movement {
    /// A list of key signatures used in this movement.
    pub sig: Vec<Sig>,
    /// Each measure of the movement in order.
    pub bar: Vec<Bar>,
}

impl Default for Movement {
    fn default() -> Movement {
        muon::from_str(include_str!("default_movement.muon")).unwrap()
    }
}

/// An instrument in the soundfont for this score.
#[derive(PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct Instrument {
    // Default waveform for instrument.
    waveform: String,
    // Straight or Palm mute depending on instrument.
    mute: Option<String>,
    // Cup mute.
    cup_mute: Option<String>,
    // Wah-wah mute.
    harmon_mute: Option<String>,
    // Plunger mute.
    plunger_mute: Option<String>,
    // Harmonic (for guitar)
    harmonic: Option<String>,

    // Use different waveform for this dynamic
    ppp: Option<String>,
    // Use different waveform for this dynamic
    pp: Option<String>,
    // Use different waveform for this dynamic
    p: Option<String>,
    // Use different waveform for this dynamic
    mp: Option<String>,
    // Use different waveform for this dynamic
    mf: Option<String>,
    // Use different waveform for this dynamic
    f: Option<String>,
    // Use different waveform for this dynamic
    ff: Option<String>,
    // Use different waveform for this dynamic
    fff: Option<String>,
}

/*/// A soundfont used in the score (either in the .scof or a .sfsf and linked to).
#[derive(PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct SoundFont {
    /// A list of instruments.
    pub instrument: Vec<Instrument>,
}*/

fn default_symtime() -> bool {
    false // Don't show a symbol for time signature.
}

/// Signature Style.
#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct SigStyle {
    /// Text that should show up.  Default="beat = BPM" marking.
    pub tempo: Option<String>,
    /// Whether or not the time signature should use a special symbol
    /// (C for 4/4).
    #[serde(default = "default_symtime")]
    pub time_symbol: bool,
    /// Text that should show up rather than default.
    /// Default="1/8 1/8 = 1/6 1/12"
    pub swing_text: Option<String>,
}

/// Style file.
#[derive(PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct Style {
    pub sig: Vec<SigStyle>,
}

/// Arranger & Ensemble
#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct Arranger {
    name: String,
    ensemble: Option<String>,
}

fn default_composer() -> String {
    "Anonymous".to_string()
}

/// Score metadata.
#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct Meta {
    /// Who wrote the original music "{}"
    #[serde(default = "default_composer")]
    pub composer: String,
    /// The subtitle of the piece.
    pub subtitle: Option<String>,
    /// Work number.
    pub number: Option<u32>,
    /// Who wrote the lyrics to the music "Words by {}"
    pub lyricist: Option<String>,
    /// Who translated the lyrics "Translated by {}"
    pub translator: Option<String>,
    /// Who performed the music "Performed by {}"
    pub performers: Option<String>,
    /// List of people who arranged & rearranged the music in order
    /// "Arranged for {} by {}".
    pub arranger: Vec<Arranger>,
    /// List of people who revised the score "Revised by {}".
    pub revised: Vec<String>,
    /// License information
    pub licenses: Vec<String>,
    /// Playing level (how hard it is to play times 2 - to allow grade 1.5 etc.).
    pub grade: Option<u8>,
    /// List of the movements in order.
    pub movement: Vec<String>,
}

impl Default for Meta {
    fn default() -> Self {
        Self {
            subtitle: None,
            number: None,
            composer: default_composer(),
            lyricist: None,
            translator: None,
            performers: None,
            arranger: vec![],
            revised: vec![],
            licenses: vec![],
            grade: None,
            movement: vec![],
        }
    }
}

/// The entire Scof zip file.
#[derive(PartialEq)]
pub struct Scof {
    /// The title of the piece.  When the zip file's name is
    /// "My Score \ Symphony No. 1.scof" => "My Score / Symphony No. 1".
    /// Maximum of 64 characters.
    pub title: String,
    /// Bytes for an RVG file (Vector(SVG), Pixel(PNG) or Picture(JPG)).
    pub cover: Option<Vec<u8>>,
    /// Metadata for the peice.
    pub meta: Meta,
    /// Rendering style.
    pub style: Style,
    /// Playback synthesis.
    pub synth: Synth,
    /// Instruments.
    pub soundfont: Vec<Instrument>,
    /// Movements for the peice.
    pub movement: Vec<Movement>,
}

impl Default for Scof {
    fn default() -> Scof {
        Scof {
            title: "Untitled Score".to_string(),
            cover: None,
            meta: Meta::default(),
            style: Style::default(),
            synth: Synth::default(),
            movement: vec![Movement::default()],
            soundfont: vec![Instrument::default()],
        }
    }
}

impl Scof {
    pub fn get(&self, bar: usize, chan: usize, curs: usize) -> Option<Marking> {
        let string = self.movement[0].bar.get(bar)?.chan[chan].notes.get(curs)?;

        // Read duration.
        let start_index = 0;
        let mut end_index = 0;
        for (i, c) in string.char_indices() {
            if !c.is_numeric() {
                end_index = i;
                break;
            }
        }

        let num = string[start_index..end_index].parse::<u8>().unwrap();

        // Read note name.
        match string.get(end_index..end_index+1)? {
            "R" => {
                Some(Marking::Note(Note {
                    pitch: None,
                    duration: (1, num),
                    articulation: None,
                }))
            }
            a => panic!("Failed to parse '{}'", a),
        }
    }
}

/*impl<R> From<R> for Scof where R: std::io::Read {
    fn from(a: R) -> Self {
        let mut rtn = Self::default();

//        let _ = muon::from_reader(a);

        rtn
    }
}*/

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
