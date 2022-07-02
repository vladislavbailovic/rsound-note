#[cfg(feature="graph")]
use note::graph;

#[cfg(feature="graph")]
fn main() -> std::io::Result<()> {
    graph::save()?;

    Ok(())
}

