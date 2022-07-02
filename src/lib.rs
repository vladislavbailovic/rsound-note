mod octave;
mod pitch_class;

const CONCERT_A_FREQ: i32 = 440;

trait Numeric {
    fn numeric(&self) -> i32;
}

trait Midi<T> where T: Numeric {
    fn midi(&self, other: &T) -> i32;
}

trait Freq<T>: Midi<T> where T: Numeric {
    fn freq(&self, other: &T) -> f64 {
        (self.raw_freq(other) * 100.0).round() / 100.0
    }

    fn raw_freq(&self, other: &T) -> f64 {
        let dist = self.midi(other) - 69;
        2.0_f64.powf(dist as f64 / 12.0) * CONCERT_A_FREQ as f64
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
