mod len;
pub use len::*;

mod value;
pub use value::*;

macro_rules! val {
    ($numerator:tt / $denominator:tt T) => {
        Value::from($numerator, $denominator, Some(1))
    };
    ($numerator:tt / $denominator:tt) => {
        Value::from($numerator, $denominator, None)
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn val_macro_len() {
        assert_eq!(val![2 / 1], Value::Len(Len::Double));
        assert_eq!(val![1 / 1], Value::Len(Len::Whole));
        assert_eq!(val![1 / 4], Value::Len(Len::Quarter));
        assert_eq!(val![1 / 8], Value::Len(Len::Eighth));
        assert_eq!(val![1 / 16], Value::Len(Len::Sixteenth));
        assert_eq!(val![1 / 32], Value::Len(Len::Thirtysecond));
        assert_eq!(val![1 / 64], Value::Len(Len::Sixtyfourth));
    }

    #[test]
    fn val_macro_dot() {
        assert_eq!(val![2/1 T], Value::Dot(Len::Double));
        assert_eq!(val![1/1 T], Value::Dot(Len::Whole));
        assert_eq!(val![1/4 T], Value::Dot(Len::Quarter));
        assert_eq!(val![1/8 T], Value::Dot(Len::Eighth));
        assert_eq!(val![1/16 T], Value::Dot(Len::Sixteenth));
        assert_eq!(val![1/32 T], Value::Dot(Len::Thirtysecond));
        assert_eq!(val![1/64 T], Value::Dot(Len::Sixtyfourth));
    }
}
