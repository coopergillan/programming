use std::f64::consts::PI;

enum Shape {
    Rectangle { width: f64, height: f64 },
    Circle { radius: f64 },
    Triangle { base: f64, height: f64 },
}

impl Shape {
    fn perimeter(&self) -> f64 {
        match self {
            Shape::Rectangle { width, height } => (width * 2.0) + (height * 2.0),
            Shape::Circle { radius } => 2.0 * PI * radius,
            Shape::Triangle { base: _, height: _ } => {
                println!("Unable to calculate perimeter of triangle from base and height!");
                -1.0
            }
        }
    }
    fn area(&self) -> f64 {
        match self {
            Shape::Rectangle { width, height } => width * height,
            Shape::Circle { radius } => PI * (radius * radius),
            Shape::Triangle { base, height } => 0.5 * base * height,
        }
    }
}

fn main() {
    println!("Hello, world! All the action is in the tests.");
}

// Taken from the following StackOverflow answer:
// https://stackoverflow.com/questions/34662713/how-can-i-create-parameterized-tests-in-rust
// A little surprised that this is the only way to actually create separate tests
macro_rules! area_tests {
    ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (input, expected) = $value;
                check_area(input, expected);
            }
        )*
    }
}

macro_rules! perimeter_tests {
    ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (input, expected) = $value;
                check_perimeter(input, expected);
            }
        )*
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_area(shape: Shape, want: f64) {
        let got = shape.area();
        assert_eq!(got, want, "got {:.2}, but want {:.2}", got, want);
    }

    fn check_perimeter(shape: Shape, want: f64) {
        let got = shape.perimeter();
        assert_eq!(got, want, "got {:.2}, but want {:.2}", got, want);
    }

    area_tests! {
        rectangle_area: (Shape::Rectangle { width: 12.0, height: 6.0, }, 72.0),
        circle_area: (Shape::Circle { radius: 6.0 }, 113.09733552923255),
        triangle_area: (Shape::Triangle { base: 6.0, height: 12.0 }, 36.0),
    }

    perimeter_tests! {
        rectangle_perimeter: (Shape::Rectangle { width: 12.0, height: 6.0, }, 36.0),
        circle_perimeter: (Shape::Circle { radius: 6.0 }, 37.69911184307752),
        triangle_perimeter: (Shape::Triangle { base: 6.0, height: 12.0 }, -1.0),
    }
}
