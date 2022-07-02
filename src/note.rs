use crate::{Octave, PitchClass, Value};
use crate::Midi;

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

