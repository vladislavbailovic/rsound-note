use crate::octave::*;
use crate::{Numeric, Midi, Freq};

#[derive(Copy, Clone, Debug)]
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
