use crate::pitch_class::*;
use crate::{CONCERT_A_FREQ};

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

impl Octave {
    pub fn freq(&self, pc: &PitchClass) -> f64 {
        (self.raw_freq(pc) * 100.0).round() / 100.0
    }

    pub fn midi(&self, pc: &PitchClass) -> i32 {
        let tone_ord = *pc as i32;
        let octave_ord = *self as i32;

        127.min(tone_ord + octave_ord * 12)
    }

    pub fn raw_freq(&self, pc: &PitchClass) -> f64 {
        let dist = self.midi(pc) - 69;
        2.0_f64.powf(dist as f64 / 12.0) * CONCERT_A_FREQ as f64
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
