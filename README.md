# ScoreIMU (*.score)
Music score format developed for ScoreFall based on IMU (Icy MarkUp).

## Format
Format is a zip file.  An example contents are in `score/`.


## Language
4-space indentation.  Like MusicXML, each file represents a movement.
Use `//` for comments.

```
C // Condense 2 Staves for conductor
G // Grand Staff
M // Measure
P // Add a Part (Can get sheet music for this part)
S // Add a Section (really, a system).
T // A track (really, a staff).
```

```sm3
P Piano // add piano & alto voice part called "piano".
    T0 voice // Add voice track.
    G // Make a grand staff.
        T1 piano // Add piano track (right hand).
        T2 piano // Add piano track (left hand).
S // Add a brass section (musicians call system)
    C // Condense for Conductors' score 2 staves into 1.
        P Trombone 1
            T3 trombone // Add trombone track.
        P Trombone 2
            T4 trombone // Add trombone track.
    P Bass Trombone
        T5 trombone // Add trombone track.

// Add measure & change defaults
// - Key=C to Key=C#
// - Time=4/4 to Time=15/16
// - Tempo=120BPM to Tempo=160BPM
// - Ensemble instruction: "Aggessively"
M C# 15/16 160 "Aggressively"
    
M //Add another measure

```

### TODO
- File for musical ideas (notes - a movement)
- File for synthesis (soundfont data, instruments)
- File for formatting (page breaks, line breaks, customized spacing, font choice)
- File for metadata (composer, title, copyright, movement order)
- Answer: Do we have 4 files in the ZIP?
