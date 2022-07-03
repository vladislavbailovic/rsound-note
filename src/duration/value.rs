use crate::Len;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Value {
    Len(Len),
    Dot(Len),
}

impl Value {
    pub fn from(numerator: usize, denominator: usize, dot: Option<usize>) -> Value {
        let len = match numerator {
            1 => match denominator {
                1 => Len::Whole,
                2 => Len::Half,
                4 => Len::Quarter,
                8 => Len::Eighth,
                16 => Len::Sixteenth,
                32 => Len::Thirtysecond,
                64 => Len::Sixtyfourth,
                _ => Len::Quarter,
            },
            2 => match denominator {
                1 => Len::Double,
                _ => Len::Quarter,
            },
            _ => Len::Quarter,
        };
        match dot {
            None => Value::Len(len),
            Some(_) => Value::Dot(len),
        }
    }

    pub(crate) fn per_beat(&self) -> f32 {
        match &self {
            Value::Len(ln) => ln.per_beat(),
            Value::Dot(ln) => {
                let beats = 1.0 / ln.per_beat();
                let dot_beats = beats * 1.5;
                1.0 / dot_beats
            }
        }
    }

    fn secs(&self, bpm: f32) -> f32 {
        match &self {
            Value::Len(ln) => ln.secs(bpm),
            Value::Dot(ln) => ln.secs(bpm) * 1.5,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn value_from() {
        if let Value::Len(Len::Double) = Value::from(2, 1, None) {
            assert!(true);
        } else {
            assert!(false, "Double note not recognized");
        }
        if let Value::Len(Len::Whole) = Value::from(1, 1, None) {
            assert!(true);
        } else {
            assert!(false, "Whole note not recognized");
        }
        if let Value::Len(Len::Half) = Value::from(1, 2, None) {
            assert!(true);
        } else {
            assert!(false, "Half note not recognized");
        }
        if let Value::Len(Len::Quarter) = Value::from(1, 4, None) {
            assert!(true);
        } else {
            assert!(false, "Quarter note not recognized");
        }

        if let Value::Len(Len::Eighth) = Value::from(1, 8, None) {
            assert!(true);
        } else {
            assert!(false, "Eighth note not recognized");
        }
        if let Value::Len(Len::Sixteenth) = Value::from(1, 16, None) {
            assert!(true);
        } else {
            assert!(false, "Sixteenth note not recognized");
        }
        if let Value::Len(Len::Thirtysecond) = Value::from(1, 32, None) {
            assert!(true);
        } else {
            assert!(false, "Thirtysecond note not recognized");
        }
        if let Value::Len(Len::Sixtyfourth) = Value::from(1, 64, None) {
            assert!(true);
        } else {
            assert!(false, "Sixtyfourth note not recognized");
        }
    }

    #[test]
    fn value_beats_len() {
        assert_eq!(Value::Len(Len::Quarter).per_beat(), 1.0);
        assert_eq!(Value::Len(Len::Half).per_beat(), 0.5);
        assert_eq!(Value::Len(Len::Whole).per_beat(), 0.25);
        assert_eq!(Value::Len(Len::Double).per_beat(), 0.125);

        assert_eq!(Value::Len(Len::Eighth).per_beat(), 2.0);
        assert_eq!(Value::Len(Len::Sixteenth).per_beat(), 4.0);
        assert_eq!(Value::Len(Len::Thirtysecond).per_beat(), 8.0);
        assert_eq!(Value::Len(Len::Sixtyfourth).per_beat(), 16.0);
    }

    #[test]
    fn value_beats_dot() {
        assert_eq!(Value::Dot(Len::Quarter).per_beat(), 1.5);
        assert_eq!(Value::Dot(Len::Half).per_beat(), 0.75);
        assert_eq!(Value::Dot(Len::Whole).per_beat(), 0.375);
        assert_eq!(Value::Dot(Len::Double).per_beat(), 0.1875);

        assert_eq!(Value::Dot(Len::Eighth).per_beat(), 3.0);
        assert_eq!(Value::Dot(Len::Sixteenth).per_beat(), 6.0);
        assert_eq!(Value::Dot(Len::Thirtysecond).per_beat(), 12.0);
        assert_eq!(Value::Dot(Len::Sixtyfourth).per_beat(), 24.0);
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
