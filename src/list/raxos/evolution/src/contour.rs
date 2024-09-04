use std::fmt;

use crate::display_slice::DisplaySliceExt;
use crate::point::Point;

/// The Contour line of all the points that have the same value
///
/// This is not an accurate line.
#[derive(Clone, Debug)]
pub struct Contour {
    pub points: Vec<Point>,
}

impl fmt::Display for Contour {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.points.display_n::<30>())?;
        Ok(())
    }
}

impl Contour {
    pub fn new(points: impl IntoIterator<Item = Point>) -> Self {
        Self {
            points: points.into_iter().collect(),
        }
    }

    /// Update the contour line by adding random points or move the existing points.
    ///
    /// `prob_move` is the chance of moving a point.
    /// `1 - prob_move` is the chance of adding a new point.
    pub fn rand_update(
        &self,
        prob_move: f64,
        mut prob_add: f64,
        mut prob_remove: f64,
    ) -> (Self, String) {
        let l = self.points.len();

        if l <= 3 {
            // always move
            prob_remove = 0.0;
        }

        if l >= 40 {
            // always remove
            prob_add = 0.0;
        }

        // Generate a rand number between 0 and 1
        let rand = rand::random::<f64>() * (prob_move + prob_add + prob_remove);

        if rand < prob_move {
            // move a point randomly
            let i = (rand::random::<u64>() as usize) % self.points.len();
            if self.points[i] == Point::new(1f64, 1f64) {
                // The unit point should not be moved
                return (self.clone(), "Nothing".to_string());
            }

            let (x_left, x_right) = if i == 0 {
                let right_sibling = self.points[i + 1].x;
                (0.0, right_sibling)
            } else if i == l - 1 {
                let left_sibling = self.points[i - 1].x;
                (left_sibling, (self.points[i].x * 1.5f64))
            } else {
                (self.points[i - 1].x, self.points[i + 1].x)
            };

            let (y_low, y_high) = if i == 0 {
                let right_sibling = self.points[i + 1].y;
                (right_sibling, (self.points[i].y * 1.5f64))
            } else if i == l - 1 {
                let last = self.points[i - 1].y;
                (0.0, last)
            } else {
                (self.points[i + 1].y, self.points[i - 1].y)
            };

            let mut points = self.points.clone();

            let x = rand::random::<f64>() * (x_right - x_left) + x_left;
            let y = rand::random::<f64>() * (y_high - y_low) + y_low;
            let p = Point::new(x, y);

            let p0 = points[i];
            points[i] = p;
            (Self { points }, format!("Move {i}: from {p0} to {p}"))
        } else if rand < prob_move + prob_add {
            // Add a new point

            let p1 = Point::new(1f64, 1f64);
            let position = self.points.iter().position(|p| *p == p1).unwrap();

            let i = loop {
                let i = (rand::random::<u64>() as usize) % (self.points.len() + 1);
                if i > position && position < l / 2 || i <= position && position >= l / 2 {
                    // The unit point should not be moved
                    continue;
                }

                break i;
            };

            // The index of point before which to add new point

            let (x_left, x_right, y_low, y_high) = if i == 0 {
                let right = self.points[i];
                (0.0, right.x, right.y, right.y * 1.5)
            } else if i == self.points.len() {
                let left = self.points[i - 1];
                (left.x, left.x * 1.5, 0.0, left.y)
            } else {
                let left = self.points[i - 1];
                let right = self.points[i];
                (left.x, right.x, right.y, left.y)
            };

            let x = rand::random::<f64>() * (x_right - x_left) + x_left;
            let y = rand::random::<f64>() * (y_high - y_low) + y_low;

            let mut points = self.points.clone();
            let p = Point::new(x, y);
            points.insert(i, p);
            (Self { points }, format!("Add {p} before {i}"))
        } else {
            // remove a point
            let i = (rand::random::<u64>() as usize) % self.points.len();
            if self.points[i] == Point::new(1f64, 1f64) {
                // The unit point should not be removed
                return (self.clone(), "Nothing".to_string());
            }

            let mut points = self.points.clone();

            points.remove(i);
            (Self { points }, format!("Remove {i}"))
        }
    }

    /// Compare if a point is after or before the contour line
    pub fn below_eq(&self, p: &Point) -> bool {
        let x = self.cross_product_x(p);

        x >= 0f64
    }

    pub fn cross_product_x(&self, p: &Point) -> f64 {
        let (p1, p2) = self.find_adjacent_points_for(p);
        let a = p2 - p1;
        let b = *p - p1;

        a.cross_product(&b)
    }

    fn find_adjacent_points_for(&self, p: &Point) -> (Point, Point) {
        let first_bigger = self
            .points
            .iter()
            .enumerate()
            .filter(|(_i, q)| q.x >= p.x)
            .next();

        if let Some(first) = first_bigger {
            let index = first.0;
            if index == self.points.len() - 1 {
                // p is the last point
                (self.points[index - 1], self.points[index])
            } else if index == 0 {
                (self.points[0], self.points[1])
            } else {
                (self.points[index - 1], self.points[index])
            }
        } else {
            // p >= all points, use the last two.
            let mut it = self.points.iter().rev();
            let last = it.next().unwrap();
            let second_last = it.next().unwrap();
            (*second_last, *last)
        }
    }

    pub fn validate(&self) {
        assert!(self
            .points
            .iter()
            .position(|p| *p == Point::new(1.0, 1.0))
            .is_some());

        for i in 1..self.points.len() {
            assert!(self.points[i - 1].x < self.points[i].x, "{:?}", self.points);

            assert!(self.points[i - 1].y > self.points[i].y, "{:?}", self.points);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::contour::Contour;
    use crate::point::Point;

    #[test]
    fn test_find_adjacent() {
        let c = contour([(1, 1), (2, 2), (3, 3)]);
        assert_eq!(c.find_adjacent_points_for(&pi(0, 0)), (pi(1, 1), pi(2, 2)));
        assert_eq!(c.find_adjacent_points_for(&pi(1, 1)), (pi(1, 1), pi(2, 2)));
        assert_eq!(c.find_adjacent_points_for(&pi(1, 2)), (pi(1, 1), pi(2, 2)));
        assert_eq!(
            c.find_adjacent_points_for(&p(1.5, 2.0)),
            (pi(1, 1), pi(2, 2))
        );
        assert_eq!(c.find_adjacent_points_for(&pi(2, 2)), (pi(1, 1), pi(2, 2)));
        assert_eq!(
            c.find_adjacent_points_for(&p(2.5, 2.0)),
            (pi(2, 2), pi(3, 3))
        );
        assert_eq!(c.find_adjacent_points_for(&pi(3, 3)), (pi(2, 2), pi(3, 3)));
        assert_eq!(c.find_adjacent_points_for(&pi(4, 4)), (pi(2, 2), pi(3, 3)));
    }

    #[test]
    fn test_above() {
        assert_eq!(contour([(1, 2), (2, 3)]).below_eq(&pi(2, 2)), false);
        assert_eq!(contour([(1, 2), (2, 3)]).below_eq(&pi(2, 4)), true);

        assert_eq!(contour([(1, 2), (3, 4)]).below_eq(&pi(2, 4)), true);
        assert_eq!(contour([(1, 2), (3, 4)]).below_eq(&pi(2, 3)), true);
        assert_eq!(contour([(1, 2), (3, 4)]).below_eq(&pi(2, 2)), false);
        assert_eq!(contour([(1, 2), (3, 4)]).below_eq(&pi(2, 1)), false);

        assert_eq!(contour([(1, 2), (3, 4)]).below_eq(&pi(0, 2)), true);
        assert_eq!(contour([(1, 2), (3, 4)]).below_eq(&pi(0, 1)), true);
        assert_eq!(contour([(1, 2), (3, 4)]).below_eq(&pi(0, 0)), false);

        // 3 +
        //   |
        // 2 +       * - *
        //   |    /
        // 1 +   *
        //   |
        // 0 +---+---+---+---+---->
        //   0   1   2   3   4
        let c = contour([(1, 1), (2, 2), (3, 2)]);
        assert_eq!(c.below_eq(&pi(0, -1)), false);
        assert_eq!(c.below_eq(&pi(0, 0)), true);
        assert_eq!(c.below_eq(&pi(0, 1)), true);
        assert_eq!(c.below_eq(&pi(1, 0)), false);
        assert_eq!(c.below_eq(&pi(1, 1)), true);
        assert_eq!(c.below_eq(&pi(1, 2)), true);

        assert_eq!(c.below_eq(&pi(2, 1)), false);
        assert_eq!(c.below_eq(&pi(2, 2)), true);
        assert_eq!(c.below_eq(&pi(2, 3)), true);

        assert_eq!(c.below_eq(&pi(3, 1)), false);
        assert_eq!(c.below_eq(&pi(3, 2)), true);
        assert_eq!(c.below_eq(&pi(3, 3)), true);

        assert_eq!(c.below_eq(&pi(4, 1)), false);
        assert_eq!(c.below_eq(&pi(4, 2)), true);
        assert_eq!(c.below_eq(&pi(4, 3)), true);
    }

    fn contour(points: impl IntoIterator<Item = (u64, u64)>) -> Contour {
        Contour::new(points.into_iter().map(Point::from))
    }

    fn pi(x: i64, y: i64) -> Point {
        Point::from((x as f64, y as f64))
    }

    fn p(x: f64, y: f64) -> Point {
        Point::from((x, y))
    }
}
