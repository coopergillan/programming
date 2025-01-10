// const REPEAT_COUNT: u8 = 5;

pub fn repeat(string_to_repeat: &str, times_to_repeat: u8) -> String {
    let mut repeated = String::new();
    for _ in 0..times_to_repeat {
        repeated += string_to_repeat;
    }
    repeated
}
