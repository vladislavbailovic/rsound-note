use crate::pitch_class::*;
use crate::{Freq, Midi, Numeric};

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Octave {
    Cm1,
    C0,
    C1,
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
    C9,
}

impl Numeric for Octave {
    fn numeric(&self) -> i32 {
        *self as i32
    }
}
impl Midi<PitchClass> for Octave {
    fn midi(&self, other: &PitchClass) -> i32 {
        let tone_ord = other.numeric();
        let octave_ord = self.numeric();

        127.min(tone_ord + octave_ord * 12)
    }
}
impl Freq<PitchClass> for Octave {}

impl Octave {
    pub fn try_from_int(value: i32) -> Option<Self> {
        match value {
            0 => Some(Self::Cm1),
            1 => Some(Self::C0),
            2 => Some(Self::C1),
            3 => Some(Self::C2),
            4 => Some(Self::C3),
            5 => Some(Self::C4),
            6 => Some(Self::C5),
            7 => Some(Self::C6),
            8 => Some(Self::C7),
            9 => Some(Self::C8),
            10 => Some(Self::C9),
            _ => None,
        }
    }

    pub fn max() -> i32 {
        Self::C9.numeric()
    }

    pub fn min() -> i32 {
        Self::Cm1.numeric()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_freq() {
        assert_eq!(Octave::C4.freq(&PitchClass::A), 440.0);
        assert_eq!(Octave::C5.freq(&PitchClass::C), 523.25);
        assert_eq!(Octave::Cm1.freq(&PitchClass::A), 13.75);
        assert_eq!(Octave::C0.freq(&PitchClass::B), 29.14);
    }

    #[test]
    fn test_midi() {
        assert_eq!(Octave::C4.midi(&PitchClass::A), 69);
        assert_eq!(Octave::Cm1.midi(&PitchClass::C), 0);
        assert_eq!(Octave::C0.midi(&PitchClass::A), 21);
        assert_eq!(Octave::C9.midi(&PitchClass::B), 127);
    }
}
