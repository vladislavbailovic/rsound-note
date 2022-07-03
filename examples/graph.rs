#[cfg(feature = "graph")]
use note::graph::Roll;

use note::*;

fn get_blocks() -> Vec<(i32, f32)> {
    vec![
        note![A: C0, 1 / 4],
        note![C: C1, 1 / 4 T],
        note![A: C0, 1 / 8],
        note![B: C0, 1 / 8 T],
    ]
    .iter()
    .map(|n| {
        let y = n.midi();
        let beats = 1.0 / n.per_beat();
        let bars = beats * 4.0;
        (y, bars)
    })
    .collect()
}

#[cfg(feature = "graph")]
fn main() -> std::io::Result<()> {
    let blocks = get_blocks();

    let mut roll = Roll::new();
    roll.beats(4);
    roll.draw("foo.ppm", &blocks);

    Ok(())
}
