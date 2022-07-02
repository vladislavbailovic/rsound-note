mod duration;
pub use duration::{Len,Value};

mod octave;
pub use octave::Octave;

mod pitch_class;
pub use pitch_class::PitchClass;

#[cfg(feature = "graph")]
pub mod graph;

pub struct Note {
    pitch_class: PitchClass,
    octave: Octave,
    value: Value,
}

impl Note {
    pub fn new(pitch_class: PitchClass, octave: Octave, value: Value) -> Self {
        Self {
            pitch_class,
            octave,
            value,
        }
    }

    pub fn midi(&self) -> i32 {
        self.pitch_class.midi(&self.octave)
    }

    pub fn beats(&self) -> f32 {
        self.value.beats()
    }
}

trait Numeric {
    fn numeric(&self) -> i32;
}

trait Midi<T>
where
    T: Numeric,
{
    fn midi(&self, other: &T) -> i32;
}

trait Freq<T>: Midi<T>
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
