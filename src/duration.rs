// TODO: NoteT

#[derive(Copy, Clone, Debug)]
pub enum Duration {
    Double,
    Whole,
    Half,
    Quarter,
    Eighth,
    Sixteenth,
    Thirtysecond,
    Sixtyfourth,
}

impl Duration {
    pub(crate) fn beats(&self) -> f32 {
        let ord = *self as i32 + 1;
        2.0_f32.powf(ord as f32) / 16.0
    }

    fn secs(&self, bpm: f32) -> f32 {
        let beats = self.beats();
        60.0 / bpm / beats
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn beats() {
        assert_eq!(Duration::Quarter.beats(), 1.0);
        assert_eq!(Duration::Half.beats(), 0.5);
        assert_eq!(Duration::Whole.beats(), 0.25);
        assert_eq!(Duration::Double.beats(), 0.125);

        assert_eq!(Duration::Eighth.beats(), 2.0);
        assert_eq!(Duration::Sixteenth.beats(), 4.0);
        assert_eq!(Duration::Thirtysecond.beats(), 8.0);
        assert_eq!(Duration::Sixtyfourth.beats(), 16.0);
    }

    #[test]
    fn secs() {
        assert_eq!(Duration::Quarter.secs(60.0), 1.0);
        assert_eq!(Duration::Half.secs(60.0), 2.0);
        assert_eq!(Duration::Whole.secs(60.0), 4.0);
        assert_eq!(Duration::Double.secs(60.0), 8.0);

        assert_eq!(Duration::Eighth.secs(60.0), 0.5);
        assert_eq!(Duration::Sixteenth.secs(60.0), 0.25);
        assert_eq!(Duration::Thirtysecond.secs(60.0), 0.125);
        assert_eq!(Duration::Sixtyfourth.secs(60.0), 0.0625);
    }
}
