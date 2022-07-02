#[cfg(feature="graph")]
use note::graph;

use note::octave::*;
use note::pitch_class::*;
use note::Midi;

fn get_blocks() -> Vec<(i32, i32)> {
    vec![
        // Midi, duration
        (PitchClass::A.midi(&Octave::C0), 1),
        (PitchClass::C.midi(&Octave::C1), 3),
        (PitchClass::A.midi(&Octave::C0), 1),
        (PitchClass::C.midi(&Octave::C1), 2),
    ]
}

#[cfg(feature="graph")]
fn main() -> std::io::Result<()> {
    graph::save(&get_blocks())?;

    Ok(())
}

