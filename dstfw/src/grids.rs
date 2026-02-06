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
    bins: Option<Vec<Vec<Option<Box<GridPoint>>>>>,
}

impl Grid {
    fn insert(&mut self, x: f64, y: f64) -> bool {
        let xbin = ((x - self.x_start) / self.x_bin_width).floor() as usize;
        let ybin = ((y - self.y_start) / self.y_bin_width).floor() as usize;
        dbg!(&xbin);
        dbg!(&ybin);

        // Check that point is within the grid.
        if xbin < 0 || xbin >= self.num_x_bins {
            return false;
        }
        if ybin < 0 || ybin >= self.num_y_bins {
            return false;
        }

        // Add the point to the front of the list as a new Boxed GridPoint
        if self.bins.is_none() {
            self.bins = Some(vec![vec![None; self.num_y_bins]; self.num_x_bins]);
        }
        // If there is already a value, get a mutable reference and put the next
        // point at the front of the list
        if let Some(bins) = &mut self.bins {
            let next_point = bins[xbin][ybin].take();
            bins[xbin][ybin] = Some(Box::new(GridPoint {
                x,
                y,
                next: next_point,
            }));
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
        let grid = Grid {
            num_x_bins: 2,
            num_y_bins: 2,
            x_start: 0.0,
            x_end: 2.0,
            x_bin_width: 1.0,
            y_start: 0.0,
            y_end: 2.0,
            y_bin_width: 1.0,
            bins: None,
        };
        assert!(grid.bins.is_none());
    }

    #[test]
    fn test_single_grid_insert() {
        let mut grid = Grid {
            num_x_bins: 2,
            num_y_bins: 2,
            x_start: 0.0,
            x_end: 2.0,
            x_bin_width: 1.0,
            y_start: 0.0,
            y_end: 2.0,
            y_bin_width: 1.0,
            bins: None,
        };
        grid.insert(0.5, 0.75);

        assert_eq!(
            grid.bins.expect("Couldn't get bins")[0][0],
            Some(Box::new(GridPoint::new(0.5, 0.75)))
        );
    }

    #[test]
    fn test_complex_grid_insert() {
        let mut grid = Grid {
            num_x_bins: 2,
            num_y_bins: 2,
            x_start: 0.0,
            x_end: 2.0,
            x_bin_width: 1.0,
            y_start: 0.0,
            y_end: 2.0,
            y_bin_width: 1.0,
            bins: None,
        };
        assert!(grid.bins.is_none());

        // In the top-left-most grid: 0, 0
        grid.insert(0.5, 0.75);
        grid.insert(0.25, 0.55);
        grid.insert(0.15, 0.95);

        // Check that the latest one is put first
        assert_eq!(
            grid.bins.as_ref().unwrap()[0][0].as_ref(),
            Some(Box::new(GridPoint {
                x: 0.15,
                y: 0.95,
                next: Some(Box::new(GridPoint {
                    x: 0.25,
                    y: 0.55,
                    next: Some(Box::new(GridPoint::new(0.5, 0.75)))
                }))
            }))
            .as_ref(),
        );
        // Confirm other bins are still empty
        assert!(grid.bins.as_ref().unwrap()[1][0].is_none());
        assert!(grid.bins.as_ref().unwrap()[1][1].is_none());
        assert!(grid.bins.as_ref().unwrap()[0][1].is_none());

        // Add to some other bins
        // Bin 1, 0
        grid.insert(1.2, 0.1);
        assert_eq!(
            grid.bins.as_ref().unwrap()[1][0].as_ref(),
            Some(Box::new(GridPoint::new(1.2, 0.1))).as_ref()
        );

        // Bin 1, 1
        grid.insert(1.5, 1.14);
        assert_eq!(
            grid.bins.as_ref().unwrap()[1][1].as_ref(),
            Some(Box::new(GridPoint::new(1.5, 1.14))).as_ref()
        );

        // Bin 0, 1
        grid.insert(0.5, 1.9);
        assert_eq!(
            grid.bins.as_ref().unwrap()[0][1].as_ref(),
            Some(Box::new(GridPoint::new(0.5, 1.9))).as_ref()
        );
    }

    #[test]
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
