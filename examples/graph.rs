#[cfg(feature = "graph")]
use note::graph;

use note::*;

fn get_blocks() -> Vec<(i32, f32)> {
    vec![
        Note::new(PitchClass::A, Octave::C0, Duration::Quarter),
        Note::new(PitchClass::C, Octave::C1, Duration::Eighth),
        Note::new(PitchClass::A, Octave::C0, Duration::Sixteenth),
        Note::new(PitchClass::B, Octave::C0, Duration::Half),
    ]
    .iter()
    .map(|x| (x.midi(), 4.0 / x.beats()))
    .collect()
}

#[cfg(feature = "graph")]
fn main() -> std::io::Result<()> {
    let blocks = get_blocks();
    println!("{:?}", blocks);
    graph::save(&blocks)?;

    Ok(())
}
