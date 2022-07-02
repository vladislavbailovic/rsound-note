pub mod octave;
pub mod pitch_class;

#[cfg(feature="graph")]
pub mod graph;

pub trait Numeric {
    fn numeric(&self) -> i32;
}

pub trait Midi<T>
where
    T: Numeric,
{
    fn midi(&self, other: &T) -> i32;
}

pub trait Freq<T>: Midi<T>
where
    T: Numeric,
{
    fn freq(&self, other: &T) -> f64 {
        (self.raw_freq(other) * 100.0).round() / 100.0
    }

    fn raw_freq(&self, other: &T) -> f64 {
        let dist = self.midi(other) - 69;
        2.0_f64.powf(dist as f64 / 12.0) * 440.0
    }
}
