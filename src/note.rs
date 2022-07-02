use crate::Midi;
use crate::{Len, Octave, PitchClass, Value};

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

#[cfg(test)]
mod tests {
    use super::*;

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
