#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Len {
    Double,
    Whole,
    Half,
    Quarter,
    Eighth,
    Sixteenth,
    Thirtysecond,
    Sixtyfourth,
}

impl Len {
    pub(crate) fn beats(&self) -> f32 {
        let ord = *self as i32 + 1;
        2.0_f32.powf(ord as f32) / 16.0
    }

    pub(crate) fn secs(&self, bpm: f32) -> f32 {
        let beats = self.beats();
        60.0 / bpm / beats
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn length_beats() {
        assert_eq!(Len::Quarter.beats(), 1.0);
        assert_eq!(Len::Half.beats(), 0.5);
        assert_eq!(Len::Whole.beats(), 0.25);
        assert_eq!(Len::Double.beats(), 0.125);

        assert_eq!(Len::Eighth.beats(), 2.0);
        assert_eq!(Len::Sixteenth.beats(), 4.0);
        assert_eq!(Len::Thirtysecond.beats(), 8.0);
        assert_eq!(Len::Sixtyfourth.beats(), 16.0);
    }

    #[test]
    fn length_secs() {
        assert_eq!(Len::Quarter.secs(60.0), 1.0);
        assert_eq!(Len::Half.secs(60.0), 2.0);
        assert_eq!(Len::Whole.secs(60.0), 4.0);
        assert_eq!(Len::Double.secs(60.0), 8.0);

        assert_eq!(Len::Eighth.secs(60.0), 0.5);
        assert_eq!(Len::Sixteenth.secs(60.0), 0.25);
        assert_eq!(Len::Thirtysecond.secs(60.0), 0.125);
        assert_eq!(Len::Sixtyfourth.secs(60.0), 0.0625);
    }
}
