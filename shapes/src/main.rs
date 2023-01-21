fn main() {
    println!("Hello, world!");
}

fn perimeter(width: f64, height: f64) -> f64 {
    2.0 * (width + height)
}

fn area(width: f64, height: f64) -> f64 {
    width * height
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_perimeter() {
        let got = perimeter(10.0, 10.0);
        let want = 40.0;
        assert_eq!(got, want, "got {:.2}, wanted {:.2}", got, want);
    }

    #[test]
    fn test_area() {
        let got = area(12.0, 6.0);
        let want = 72.0;
        assert_eq!(got, want, "got {:.2}, wanted {:.2}", got, want);
    }
}

