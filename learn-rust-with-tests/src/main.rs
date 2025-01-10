mod adder;
mod iteration;

pub use crate::adder::add_them;
pub use crate::iteration::repeat;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adder() {
        let sum = add_them(2, 2);
        let expected = 4;
        assert_eq!(sum, expected);
    }

    #[test]
    fn test_repeat() {
        let repeated = repeat("a", 5);
        let expected = "aaaaa";
        assert_eq!(repeated, expected);

        let repeated = repeat("jj", 4);
        let expected = "jjjjjjjj";
        assert_eq!(repeated, expected);
    }
}
