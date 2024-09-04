use std::fmt;
use std::ops::Sub;

/// A point in the 2D space
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, " {:.3}={:.3}x{:.3}", self.x * self.y, self.x, self.y)
    }
}

impl From<(f64, f64)> for Point {
    fn from((x, y): (f64, f64)) -> Self {
        Point { x, y }
    }
}

impl From<(u64, u64)> for Point {
    fn from((x, y): (u64, u64)) -> Self {
        Point {
            x: x as f64,
            y: y as f64,
        }
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }

    pub fn cross_product(&self, rhs: &Point) -> f64 {
        self.x * rhs.y - self.y * rhs.x
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_display() {
        let p = super::Point::new(1.12345, 2.678899);
        assert_eq!(format!("{}", p), "(1.1235, 2.6789)");
    }
}
