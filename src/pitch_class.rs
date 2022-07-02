use crate::octave::*;
use crate::{CONCERT_A_FREQ, CONCERT_A_ORDER};

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

impl PitchClass {
    fn halftones_from_root(&self, oct: &Octave) -> i32 {
        let tone_ord = *self as i32;
        let octave_ord = *oct as i32;

        (tone_ord + (octave_ord - 1) * 12) - CONCERT_A_ORDER
    }

    pub fn freq(&self, oct: &Octave) -> f64 {
        (self.raw_freq(oct) * 100.0).round() / 100.0
    }

    pub fn midi(&self, oct: &Octave) -> i32 {
        let tone_ord = *self as i32;
        let octave_ord = *oct as i32;

        127.min(tone_ord + octave_ord * 12)
    }

    pub fn raw_freq(&self, oct: &Octave) -> f64 {
        let dist = self.midi(oct) - 69;
        2.0_f64.powf(dist as f64 / 12.0) * CONCERT_A_FREQ as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dist() {
        assert_eq!(PitchClass::A.halftones_from_root(&Octave::C4), 0);
        assert_eq!(PitchClass::C.halftones_from_root(&Octave::C5), 3);
        assert_eq!(PitchClass::F.halftones_from_root(&Octave::C4), -4);
    }

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
