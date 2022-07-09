use crate::Midi;
use crate::{Octave, PitchClass, Value};

#[derive(Copy, Clone, Debug)]
pub enum Note {
    Tone(PitchClass, Octave, Value),
    Rest(Value),
}

impl Note {
    pub fn midi(&self) -> Option<i32> {
        match &self {
            Self::Tone(p, o, _) => Some(p.midi(o)),
            Self::Rest(_) => None,
        }
    }

    pub fn per_beat(&self) -> f32 {
        match &self {
            Self::Tone(_, _, v) => v.per_beat(),
            Self::Rest(v) => v.per_beat(),
        }
    }
}

#[macro_export]
macro_rules! note {
    ($pc:tt : $oct:tt, $numerator:tt / $denominator:tt T) => {
        Note::Tone(PitchClass::$pc, Octave::$oct, val![$numerator/$denominator T])
    };
    ($pc:tt : $oct:tt, $numerator:tt / $denominator:tt) => {
        Note::Tone(PitchClass::$pc, Octave::$oct, val![$numerator/$denominator])
    };
    ($pc:tt : $oct:tt) => {
        Note::Tone(PitchClass::$pc, Octave::$oct, Value::Len(Len::Quarter))
    }
}

#[macro_export]
macro_rules! pause {
    ($numerator:tt / $denominator:tt) => {
        Note::Rest(val![$numerator/$denominator T])
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Len;

    #[test]
    fn note_macro_default_len() {
        let note = note![E: C4];
        assert_eq!(note.midi(), Some(Octave::C4.midi(&PitchClass::E)));
        assert_eq!(note.per_beat(), Value::Len(Len::Quarter).per_beat());
    }

    #[test]
    fn note_macro_explicit_len() {
        let note = note![E: C4, 1 / 2];
        assert_eq!(note.midi(), Some(Octave::C4.midi(&PitchClass::E)));
        assert_eq!(note.per_beat(), Value::Len(Len::Half).per_beat());
    }

    #[test]
    fn note_macro_explicit_len_t() {
        let note = note![E:C4, 1/2 T];
        assert_eq!(note.midi(), Some(Octave::C4.midi(&PitchClass::E)));
        assert_eq!(note.per_beat(), Value::Dot(Len::Half).per_beat());
    }
}
