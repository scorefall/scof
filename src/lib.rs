/// Signature Style.
pub struct SigStyle {
    /// Whether or not the time signature should use a special symbol
    /// (C for 4/4).  Default=false
    pub time_symbol: bool,
    /// Text that should show up.  Default="beat = BPM" marking.
    pub tempo: String,
    /// Text that should show up rather than default.
    /// Default="eighth eighth = triplet quarter, triple eighth"
    pub swing_text: String,
}

/// Style file.
pub struct Style {
    pub sig: Vec<SigStyle>,
    pub bar: Vec<Bar>,
}

/// Synthesis file.
pub struct Synth {
    // TODO
}

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

/// A signature.
pub struct Sig {
    /// The key signature.
    pub key: Vec<PitchClass>,
    /// Time signature (num_beats/note_len), 4/4 is common.
    pub time: (u8, u8),
    /// BPM (beats per minute), 120 is common.
    pub tempo: u16,
    /// % Swing (default=50).
    pub swing: u8,
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
    pub pitch: PitchClass,
    pub octave: i8,
    pub duration: (u8, u8),
    pub articulation: Articulation,
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

/// Channel information for a specific bar of music.
pub struct Chan(Vec<(Marking, Option<String>)>);

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

/// A bar (or measure) of music.
pub struct Bar {
    /// All of the channels in this piece.
    pub chan: Vec<Chan>,
    /// 
    pub repeat: Vec<Repeat>,
}

/// A movement in the score.
pub struct Movement {
    /// A list of key signatures used in this movement.
    pub sig: Vec<Sig>,
    /// Each measure of the movement in order.
    pub bar: Vec<Bar>,
}

/// A parsed SCOF file.
pub struct Scof {
    /// The title of the piece.
    pub title: String,
    ///The subtitle of the piece.
    pub subtitle: String,
    /// Work number.
    pub number: u32,
    /// Who wrote the music.
    pub music: String,
    /// Who wrote the lyrics to the music.
    pub words: String,
    /// Who translated the lyrics "Translated by {}"
    pub translator: String,
    /// Who performed the music "Performed by {}"
    pub performers: String,
    /// List of people who arranged & rearranged the music in order.
    pub arranger: Vec<(String, String)>,
    /// List of people who revised the score "Revised by {}".
    pub revised: Vec<String>,
    /// List of licenses that apply to this score.
    pub license: Vec<String>,
    /// Score notes - usually for conductor (Markdown)
    pub notes: String,
    /// Grade of the peice * 2.
    pub grade: u32,
    /// Cover picture (SVG).
    pub cover: String,
    /// List of the movements name & data in order.
    pub movement: Vec<(String, Movement)>,
    /// Rendering style.
    pub style: Style,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
