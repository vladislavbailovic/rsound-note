use crate::octave::*;
use crate::{Freq, Midi, Numeric};

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PitchClass {
    C,
    Cis,
    D,
    Dis,
    E,
    F,
    Fis,
    G,
    Gis,
    A,
    B,
    H,
}

impl Numeric for PitchClass {
    fn numeric(&self) -> i32 {
        *self as i32
    }
}
impl Midi<Octave> for PitchClass {
    fn midi(&self, other: &Octave) -> i32 {
        let tone_ord = self.numeric();
        let octave_ord = other.numeric();

        127.min(tone_ord + octave_ord * 12)
    }
}
impl Freq<Octave> for PitchClass {}

impl PitchClass {
    pub fn try_from_int(value: i32) -> Option<Self> {
        match value {
            0 => Some(Self::C),
            1 => Some(Self::Cis),
            2 => Some(Self::D),
            3 => Some(Self::Dis),
            4 => Some(Self::E),
            5 => Some(Self::F),
            6 => Some(Self::Fis),
            7 => Some(Self::G),
            8 => Some(Self::Gis),
            9 => Some(Self::A),
            10 => Some(Self::B),
            11 => Some(Self::H),
            _ => None,
        }
    }

    pub fn max() -> i32 {
        12
    }

    pub fn min() -> i32 {
        0
    }
}

impl From<i32> for PitchClass {
    fn from(x: i32) -> Self {
        Self::try_from_int(x).unwrap_or(Self::A)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_freq() {
        assert_eq!(PitchClass::A.freq(&Octave::C4), 440.0);
        assert_eq!(PitchClass::C.freq(&Octave::C5), 523.25);
        assert_eq!(PitchClass::A.freq(&Octave::Cm1), 13.75);
        assert_eq!(PitchClass::B.freq(&Octave::C0), 29.14);
    }

    #[test]
    fn test_midi() {
        assert_eq!(PitchClass::A.midi(&Octave::C4), 69);
        assert_eq!(PitchClass::C.midi(&Octave::Cm1), 0);
        assert_eq!(PitchClass::A.midi(&Octave::C0), 21);
        assert_eq!(PitchClass::B.midi(&Octave::C9), 127);
    }
}
