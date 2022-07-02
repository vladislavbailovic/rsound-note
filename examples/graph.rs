#[cfg(feature = "graph")]
use note::graph::Roll;

use note::*;

fn get_blocks() -> Vec<(i32, f32)> {
    vec![
        note![A: C0, 1 / 4],
        note![C: C1, 1 / 8],
        note![A: C0, 1 / 16],
        note![B: C0, 1 / 2],
    ]
    .iter()
    .map(|x| (x.midi(), 4.0 / x.beats()))
    .collect()
}

#[cfg(feature = "graph")]
fn main() -> std::io::Result<()> {
    let blocks = get_blocks();

    let roll = Roll::new();
    roll.draw("foo.ppm", &blocks);

    Ok(())
}
