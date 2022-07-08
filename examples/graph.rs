#[cfg(feature = "graph")]
use note::graph::Roll;

use note::*;

macro_rules! sequence {
    ($($x:expr),+) => {
        {
            let mut sequence: Vec<Box<dyn Notation>> = Vec::new();
            $(sequence.push(Box::new($x));)*
            sequence
        }
    }
}

fn get_blocks() -> Vec<(Option<i32>, f32)> {
    sequence![
        note![A: C0, 1 / 4],
        pause![1 / 14],
        note![C: C1, 1 / 4 T],
        pause![1 / 14],
        note![A: C0, 1 / 8],
        pause![1 / 14],
        note![B: C0, 1 / 8 T]
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
