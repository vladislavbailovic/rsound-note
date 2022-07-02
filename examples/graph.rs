#[cfg(feature = "graph")]
use note::graph;

use note::*;

fn get_blocks() -> Vec<(i32, f32)> {
    vec![
        Note::new(PitchClass::A, Octave::C0, Value::Len(Len::Quarter)),
        Note::new(PitchClass::C, Octave::C1, Value::Len(Len::Eighth)),
        Note::new(PitchClass::A, Octave::C0, Value::Len(Len::Sixteenth)),
        Note::new(PitchClass::B, Octave::C0, Value::Len(Len::Half)),
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
