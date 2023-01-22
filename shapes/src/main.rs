struct Rectangle {
    width: f64,
    height: f64,
}

struct Circle {
    radius: f64,
}

enum Shape {
    Rectangle { width: f64, height: f64 },
    Circle { radius: f64 },
}

impl Shape {
    fn perimeter(&self) -> f64 {
        match *self {
            Shape::Rectangle { width, height } => (width * 2.0) + (height * 2.0),
            Shape::Circle { radius } => 2.0 * 3.14159265358979 * radius,
            _ => 0.0,
        }
    }
    fn area(&self) -> f64 {
        match self {
            Shape::Rectangle { width, height } => width * height,
            Shape::Circle { radius } => 3.14159265358979 * (radius * radius),
            _ => 0.0,
        }
    }
}

fn main() {
    println!("Hello, world!");
}

// fn perimeter(rectangle: Rectangle) -> f64 {
//     2.0 * (rectangle.width + rectangle.height)
// }

// fn area(rectangle: Rectangle) -> f64 {
//     rectangle.width * rectangle.height
// }
//
// fn area(circle: Circle) -> f64 {
//     3.14159265358979 * (circle.radius * circle.radius)
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rectangle_perimeter() {
        let rectangle = Shape::Rectangle {
            width: 10.0,
            height: 10.0,
        };
        let got = rectangle.perimeter();
        let want = 40.0;
        assert_eq!(got, want, "got {:.2}, wanted {:.2}", got, want);
    }

    #[test]
    fn test_rectangle_area() {
        let rectangle = Shape::Rectangle { width: 12.0, height: 6.0 };
        let got = rectangle.area();
        let want = 72.0;
        assert_eq!(got, want, "got {:.2}, wanted {:.2}", got, want);
    }

    #[test]
    #[ignore]
    fn test_circle_area() {
        let circle = Shape::Circle { radius: 6.0 };
        let got = circle.area();
        let want = 314.1592653589793;
        assert_eq!(got, want, "got {:.20}, wanted {:.20}", got, want);
    }
}
