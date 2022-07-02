#[cfg(feature = "graph")]
use note::graph;

use note::*;

fn get_blocks() -> Vec<(i32, i32)> {
    vec![
        // Midi, duration
        (Note::new(PitchClass::A, Octave::C0).midi(), 1),
        (Note::new(PitchClass::C, Octave::C1).midi(), 3),
        (Note::new(PitchClass::A, Octave::C0).midi(), 1),
        (Note::new(PitchClass::C, Octave::C1).midi(), 2),
    ]
}

#[cfg(feature = "graph")]
fn main() -> std::io::Result<()> {
    graph::save(&get_blocks())?;

    Ok(())
}
