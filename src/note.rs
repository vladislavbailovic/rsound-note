use crate::Midi;
use crate::{Octave, PitchClass, Value};

pub trait Notation {
    fn midi(&self) -> Option<i32>;
    fn per_beat(&self) -> f32;
}

#[derive(Debug)]
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
}

impl Notation for Note {
    fn midi(&self) -> Option<i32> {
        Some(self.pitch_class.midi(&self.octave))
    }

    fn per_beat(&self) -> f32 {
        self.value.per_beat()
    }
}

#[derive(Debug)]
pub struct Pause {
    value: Value,
}

impl Pause {
    pub fn new(value: Value) -> Self {
        Self { value }
    }
}

impl Notation for Pause {
    fn midi(&self) -> Option<i32> {
        None
    }
    fn per_beat(&self) -> f32 {
        self.value.per_beat()
    }
}

#[macro_export]
macro_rules! note {
    ($pc:tt : $oct:tt, $numerator:tt / $denominator:tt T) => {
        Note::new(PitchClass::$pc, Octave::$oct, val![$numerator/$denominator T])
    };
    ($pc:tt : $oct:tt, $numerator:tt / $denominator:tt) => {
        Note::new(PitchClass::$pc, Octave::$oct, val![$numerator/$denominator])
    };
    ($pc:tt : $oct:tt) => {
        Note::new(PitchClass::$pc, Octave::$oct, Value::Len(Len::Quarter))
    }
}

#[macro_export]
macro_rules! pause {
    ($numerator:tt / $denominator:tt) => {
        Pause::new(val![$numerator/$denominator T])
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Len;

    #[test]
    fn note_macro_default_len() {
        let note = note![E: C4];
        assert_eq!(note.pitch_class, PitchClass::E);
        assert_eq!(note.octave, Octave::C4);
        assert_eq!(note.value, Value::Len(Len::Quarter));
    }

    #[test]
    fn note_macro_explicit_len() {
        let note = note![E: C4, 1 / 2];
        assert_eq!(note.pitch_class, PitchClass::E);
        assert_eq!(note.octave, Octave::C4);
        assert_eq!(note.value, Value::Len(Len::Half));
    }

    #[test]
    fn note_macro_explicit_len_t() {
        let note = note![E:C4, 1/2 T];
        assert_eq!(note.pitch_class, PitchClass::E);
        assert_eq!(note.octave, Octave::C4);
        assert_eq!(note.value, Value::Dot(Len::Half));
    }
}
