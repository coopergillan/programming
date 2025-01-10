mod adder;
mod arrays_and_slices;
mod iteration;

pub use crate::adder::add_them;
pub use crate::arrays_and_slices::*;
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

    #[test]
    fn test_sum_any_number() {
        let numbers = vec![1, 2, 3, 4, 5, 6];

        let summed = sum_them(&numbers);
        let expected = 21;
        assert_eq!(summed, expected);
    }

    #[test]
    fn test_sum_all() {
        let got = sum_all(vec![vec![1, 2], vec![0, 9]]);
        let want = vec![3, 9];
        assert_eq!(got, want);
    }
}
