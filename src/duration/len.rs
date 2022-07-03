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
    pub(crate) fn per_beat(&self) -> f32 {
        let ord = *self as i32 + 1;
        2.0_f32.powf(ord as f32) / 16.0
    }

    pub(crate) fn secs(&self, bpm: f32) -> f32 {
        let beats = self.per_beat();
        60.0 / bpm / beats
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn length_per_beat() {
        assert_eq!(Len::Quarter.per_beat(), 1.0);
        assert_eq!(Len::Half.per_beat(), 0.5);
        assert_eq!(Len::Whole.per_beat(), 0.25);
        assert_eq!(Len::Double.per_beat(), 0.125);

        assert_eq!(Len::Eighth.per_beat(), 2.0);
        assert_eq!(Len::Sixteenth.per_beat(), 4.0);
        assert_eq!(Len::Thirtysecond.per_beat(), 8.0);
        assert_eq!(Len::Sixtyfourth.per_beat(), 16.0);
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
