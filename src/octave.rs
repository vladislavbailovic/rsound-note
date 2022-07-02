use crate::pitch_class::*;
use crate::{Freq, Midi, Numeric};

#[derive(Copy, Clone, Debug)]
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
