mod adder;

pub use crate::adder::add_them;

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
}
