#[cfg(feature = "graph")]
use note::graph::Roll;

use note::*;

struct Sequence {
    seq: Vec<Note>,
}

use std::ops::Deref;
impl Deref for Sequence {
    type Target = Vec<Note>;

    fn deref(&self) -> &Self::Target {
        &self.seq
    }
}

impl Sequence {
    pub fn new(seq: Vec<Note>) -> Self {
        Sequence { seq }
    }

    pub fn humanize(&mut self) -> &mut Self {
        for x in &self.seq {
            // TODO: actually humanize the sequence
            eprintln!("{:#?}", x.midi());
        }
        self
    }
}

fn get_blocks() -> Vec<(Option<i32>, f32)> {
    Sequence::new(vec![
        note![A: C0, 1 / 4],
        pause![1 / 14],
        note![C: C1, 1 / 4 T],
        pause![1 / 14],
        note![A: C0, 1 / 8],
        pause![1 / 14],
        note![B: C0, 1 / 8 T],
    ])
    .humanize()
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
