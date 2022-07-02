mod octave;
mod pitch_class;

const CONCERT_A_ORDER: i32 = 57;
const CONCERT_A_FREQ: i32 = 440;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
