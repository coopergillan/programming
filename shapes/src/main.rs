struct Rectangle {
    width: f64,
    height: f64,
}

fn main() {
    println!("Hello, world!");
}

fn perimeter(rectangle: Rectangle) -> f64 {
    2.0 * (rectangle.width + rectangle.height)
}

fn area(rectangle: Rectangle) -> f64 {
    rectangle.width * rectangle.height
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_perimeter() {
        let rectangle = Rectangle { width: 10.0, height: 10.0 };
        let got = perimeter(rectangle);
        let want = 40.0;
        assert_eq!(got, want, "got {:.2}, wanted {:.2}", got, want);
    }

    #[test]
    fn test_area() {
        let rectangle = Rectangle { width: 12.0, height: 6.0 };
        let got = area(rectangle);
        let want = 72.0;
        assert_eq!(got, want, "got {:.2}, wanted {:.2}", got, want);
    }
}

