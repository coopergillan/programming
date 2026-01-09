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

#[derive(Clone, Debug, PartialEq)]
struct GridPoint {
    x: f64,
    y: f64,
    next: Option<Box<GridPoint>>,
}

impl GridPoint {
    fn new(x: f64, y: f64) -> Self {
        GridPoint {
            x: x,
            y: y,
            next: None,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Grid {
    num_x_bins: usize,
    num_y_bins: usize,
    x_start: f64,
    x_end: f64,
    x_bin_width: f64,
    y_start: f64,
    y_end: f64,
    y_bin_width: f64,
    bins: Vec<Vec<Box<GridPoint>>>,
}

impl Grid {
    fn insert(&mut self, x: f64, y: f64) -> bool {
        let xbin = ((x - self.x_start) / self.x_bin_width).floor() as usize;
        let ybin = ((y - self.y_start) / self.y_bin_width).floor() as usize;
        dbg!(&xbin);
        dbg!(&ybin);

        // Check that point is within the grid.
        if xbin < 0 || xbin >= self.num_x_bins {
            return false
        }
        if ybin < 0 || ybin >= self.num_y_bins {
            return false
        }

        // Add the point to the front of the list as a new Boxed GridPoint
        if let Some(row) = self.bins.get_mut(xbin) {
            if let Some(val) = row.get_mut(ybin) {
                let next_point = val.clone();
                *val = Box::new(GridPoint::new(x, y));
                val.next = Some(next_point);
            }
        }

        true
    }
}

pub fn linear_scan_closest_neighbor(points: Vec<isize>, target: isize) -> Option<isize> {
    let number_of_elements = points.len();

    if number_of_elements == 0 {
        return None;
    }

    let mut candidate = points[0];
    let mut closest_distance = (target - candidate).abs();

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
    fn test_gridpoint_basics() {
        let mut first = GridPoint::new(1.0, 2.0);
        assert_eq!(first.next, None);

        let second = GridPoint::new(0.5, 1.5);
        first.next = Some(Box::new(second));
        let checked_next = first.next.unwrap();
        assert_eq!(checked_next, Box::new(GridPoint::new(0.5, 1.5)));
    }

    #[test]
    fn test_grid_basics() {
        let bins: Vec<Vec<Box<GridPoint>>> = Vec::new();
        let grid = Grid {
            num_x_bins: 2,
            num_y_bins: 2,
            x_start: 0.0,
            x_end: 2.0,
            x_bin_width: 1.0,
            y_start: 0.0,
            y_end: 2.0,
            y_bin_width: 1.0,
            bins: bins,
        };
        assert_eq!(grid.bins.len(), 0);
    }

    #[test]
    fn test_grid_insert() {
        let mut grid = Grid {
            num_x_bins: 2,
            num_y_bins: 2,
            x_start: 0.0,
            x_end: 2.0,
            x_bin_width: 1.0,
            y_start: 0.0,
            y_end: 2.0,
            y_bin_width: 1.0,
            bins: vec![vec![]],
        };
        grid.insert(0.5, 0.75);

        assert_eq!(grid.bins[0][0], Box::new(GridPoint::new(0.5, 0.75)));
    }

    #[test]
    #[ignore]
    fn test_linear_scan() {
        // Check for a target of 6
        // -1    1    6(T)         56         92
        //  |----|----|-------------|----------|
        assert_eq!(
            linear_scan_closest_neighbor(vec![-1, 1, 56, 92], 6),
            Some(1)
        );
        // Number line for points: -11, -5, 26, 92; target: 11
        //
        // -11   -5         11(T)        26         92
        //  |-----|----------|-----------|----------|
        //        |          |           |
        //        |          |           +-- 26 (distance 15 from 11)
        //        |          +-------------- 11 (target)
        //        +------------------------ -5 (distance 16 from 11)
        // +------------------------------- -11 (distance 22 from 11)
        //
        // 26 is closest to 11, so it is chosen.
        assert_eq!(
            linear_scan_closest_neighbor(vec![-11, -5, 26, 92], 11),
            Some(26)
        );
        // Number line for points: 1, 5, 9; target: 7
        //
        // 1     5     7(T)    9
        // |-----|-----|-------|
        //       |     |       |
        //       |     |       +-- 9 (distance 2 from 7)
        //       |     +---------- 7 (target)
        //       +---------------- 5 (distance 2 from 7)
        // +---------------------- 1 (distance 6 from 7)
        //
        // Both 5 and 9 are equally close to 7, but 5 appears first in the list, so it is chosen.
        assert_eq!(linear_scan_closest_neighbor(vec![1, 5, 9], 7), Some(5));
    }
}
