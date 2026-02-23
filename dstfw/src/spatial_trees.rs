//!
//!  SSSS   PPPP    AA   TTTTT  III  AA   LL
//! S      P   P  A  A    T     I  A  A  L
//!  SSS   PPPP   AAAAA    T     I AAAAA L
//!     S  P      A   A    T     I A   A L
//! SSSS   P      A   A    T    III A   A LLLLL
//!
//!      TTTTT  RRRR   EEEE  EEEE   SSSS
//!        T    R   R  E     E     S
//!        T    RRRR   EEEE  EEEE   SSS
//!        T    R R    E     E        S
//!        T    R  RR  EEEE  EEEE  SSSS
//!
//!         ^
//!        / \      Spatial Trees
//!       /   \
//!      /     \
//!     /_______\
//!        | |
//!        | |
//!
//! Taken from Data Structures the Fun Way

struct Point {
    x: f64,
    y: f64,
}

struct QuadTreeNode {
    is_leaf: bool,
    num_points: usize,
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
    children: Option<Vec<Vec<Option<Box<QuadTreeNode>>>>>,
    points: Option<Vec<Point>>,
}

impl QuadTreeNode {
    fn insert(&mut self, x: f64, y: f64) -> bool {
        // Adding another point so increment this
        node.num_points = node.num_points + 1;

        // Determine into which child bin the point should go.
        let x_bin_size: f64 = (self.x_max - self.x_min) / 2.0;
        dbg!(&x_bin_size);
        let y_bin_size: f64 = (self.y_max - self.y_min) / 2.0;
        dbg!(&y_bin_size);
        let xbin = ((x - self.x_min) / x_bin_size).floor() as usize;
        dbg!(&x_bin);
        let ybin = ((y - self.y_min) / y_bin_size).floor() as usize;
        dbg!(&y_bin);

        // Add the point to the correct child
        if !self.is_leaf {
            if self.children[xbin][ybin].is_none() {
                self.children[xbin][ybin] = QuadTreeNode {
                    is_leaf: true,
                    num_points: 1,
                    x_min: self.x_min + xbin * x_bin_size,
                    x_max: self.x_min + (xbin + 1) * x_bin_size,
                    y_min: self.y_min + ybin * y_bin_size,
                    y_max: self.y_min + (ybin + 1) * y_bin_size,
                    children: None,
                    points: None,
                };
                self.children[xbin][ybin].insert(x, y);
                return true;
            }
        }
        self.points.expect("Couldn't get points").push(Point { x, y });
        true
    }
}

struct QuadTree {
    root: QuadTreeNode,
}

impl QuadTree {
    fn insert(&mut self, x: f64, y: f64) -> bool {
        if x < self.root.x_min || x > self.root.x_max {
            return false;
        }
        if y < self.root.y_min || y > self.root.y_max {
            return false;
        }
        self.root.insert(x, y);
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quad_tree() {
        let point1 = Point { x: -1.5, y: -0.5 };
        let point2 = Point { x: 1.25, y: 0.25 };
        let point3 = Point { x: 0.85, y: -0.35 };
        let point4 = Point { x: 0.85, y: -0.55 };

        let node1 = QuadTreeNode {
            is_leaf: true,
            num_points: 4,
            x_min: -2.0,
            x_max: 2.0,
            y_min: -2.0,
            y_max: 2.0,
            children: vec![vec![None]],
            points: Some(vec![point1, point2, point3, point4]),
        };
        assert_eq!(node1.num_points, 4);
    }
}
