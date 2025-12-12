//!
//!
//!  GGG
//! G   G  RRRR   III  DDDD   SSS
//! G      R   R   I   D   D S
//! G  GG  RRRR    I   D   D  SSS
//! G   G  R R     I   D   D     S
//!  GGG   R  RR  III  DDDD   SSS
//!
//! +---+---+---+---+---+
//! | G | R | I | D | S |
//! +---+---+---+---+---+
//!
//! Taken from Data Structures the Fun Way

pub fn linear_scan_closest_neighbor(points: Vec<isize>, target: isize) -> Option<isize> {
    let number_of_elements = points.len();

    if number_of_elements == 0 {
        return None;
    }

    let mut candidate = points[0];
    dbg!(candidate);
    let mut closest_distance = (target - candidate).abs();
    dbg!(closest_distance);

    let mut i = 1;
    while i < number_of_elements {
        let current_distance = (target - points[i]).abs();
        if current_distance < closest_distance {
            closest_distance = current_distance;
            candidate = points[i];
        }
        i += 1
    }

    Some(candidate)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_scan() {
        // Check for a target of 6
        // -1    1    6(T)         56         92
        //  |----|----|-------------|----------|
        assert_eq!(
            linear_scan_closest_neighbor(vec![-1, 1, 56, 92], 6),
            Some(1)
        );
        assert_eq!(
            linear_scan_closest_neighbor(vec![-11, -5, 26, 92], 11),
            Some(26)
        );
        // Both of the last two are close enough, but the first one already meets the conditions
        assert_eq!(linear_scan_closest_neighbor(vec![1, 5, 9], 7), Some(5));
    }
}
