/// An articulation (affects how the note is played).
#[derive(Copy, Clone)]
pub enum Articulation {
    /// Really separated.
    Staccatissimo,
    /// Separated (short 1/2)
    Staccato,
    /// Tenuto
    Tenuto,
    /// Marcato (short sharp attack) (2/3)
    Marcato,
    /// Accent (sharp attack)
    Accent,
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
