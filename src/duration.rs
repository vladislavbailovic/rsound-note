#[derive(Copy, Clone, Debug)]
pub enum Value {
    Len(Len),
    Dot(Len),
}

impl Value {
    pub(crate) fn beats(&self) -> f32 {
        match &self {
            Value::Len(ln) => ln.beats(),
            Value::Dot(ln) => ln.beats() * 1.5,
        }
    }

    fn secs(&self, bpm: f32) -> f32 {
        match &self {
            Value::Len(ln) => ln.secs(bpm),
            Value::Dot(ln) => ln.secs(bpm) * 1.5,
        }
    }
}

#[derive(Copy, Clone, Debug)]
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

    fn secs(&self, bpm: f32) -> f32 {
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

    #[test]
    fn value_beats_len() {
        assert_eq!(Value::Len(Len::Quarter).beats(), 1.0);
        assert_eq!(Value::Len(Len::Half).beats(), 0.5);
        assert_eq!(Value::Len(Len::Whole).beats(), 0.25);
        assert_eq!(Value::Len(Len::Double).beats(), 0.125);

        assert_eq!(Value::Len(Len::Eighth).beats(), 2.0);
        assert_eq!(Value::Len(Len::Sixteenth).beats(), 4.0);
        assert_eq!(Value::Len(Len::Thirtysecond).beats(), 8.0);
        assert_eq!(Value::Len(Len::Sixtyfourth).beats(), 16.0);
    }

    #[test]
    fn value_beats_dot() {
        assert_eq!(Value::Dot(Len::Quarter).beats(), 1.5);
        assert_eq!(Value::Dot(Len::Half).beats(), 0.75);
        assert_eq!(Value::Dot(Len::Whole).beats(), 0.375);
        assert_eq!(Value::Dot(Len::Double).beats(), 0.1875);

        assert_eq!(Value::Dot(Len::Eighth).beats(), 3.0);
        assert_eq!(Value::Dot(Len::Sixteenth).beats(), 6.0);
        assert_eq!(Value::Dot(Len::Thirtysecond).beats(), 12.0);
        assert_eq!(Value::Dot(Len::Sixtyfourth).beats(), 24.0);
    }

    #[test]
    fn value_secs_len() {
        assert_eq!(Value::Len(Len::Quarter).secs(60.0), 1.0);
        assert_eq!(Value::Len(Len::Half).secs(60.0), 2.0);
        assert_eq!(Value::Len(Len::Whole).secs(60.0), 4.0);
        assert_eq!(Value::Len(Len::Double).secs(60.0), 8.0);

        assert_eq!(Value::Len(Len::Eighth).secs(60.0), 0.5);
        assert_eq!(Value::Len(Len::Sixteenth).secs(60.0), 0.25);
        assert_eq!(Value::Len(Len::Thirtysecond).secs(60.0), 0.125);
        assert_eq!(Value::Len(Len::Sixtyfourth).secs(60.0), 0.0625);
    }

    #[test]
    fn value_secs_dot() {
        assert_eq!(Value::Dot(Len::Quarter).secs(60.0), 1.5);
        assert_eq!(Value::Dot(Len::Half).secs(60.0), 3.0);
        assert_eq!(Value::Dot(Len::Whole).secs(60.0), 6.0);
        assert_eq!(Value::Dot(Len::Double).secs(60.0), 12.0);

        assert_eq!(Value::Dot(Len::Eighth).secs(60.0), 0.75);
        assert_eq!(Value::Dot(Len::Sixteenth).secs(60.0), 0.375);
        assert_eq!(Value::Dot(Len::Thirtysecond).secs(60.0), 0.1875);
        assert_eq!(Value::Dot(Len::Sixtyfourth).secs(60.0), 0.09375);
    }
}
