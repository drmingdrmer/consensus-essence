use crate::point::Point;

pub struct Scene {
    pub points: Vec<Point>,
}

impl Scene {
    fn new() -> Self {
        Self { points: Vec::new() }
    }

    /// Create a Scene with random points in specified range
    pub fn rand_scene(x: f64, y: f64, n: usize) -> Scene {
        let mut scene = Scene::new();
        for _ in 0..n {
            let xx = rand::random::<f64>() * x;
            let yy = rand::random::<f64>() * y;
            scene.add_point(xx, yy);
        }
        scene
    }

    fn add_point(&mut self, x: f64, y: f64) {
        self.points.push(Point { x, y });
    }

    /// Create a new Scene by applying a non-uniform scaling transformation
    /// that maps the given point to (1,1) and scales all other points accordingly.
    ///
    /// # Arguments
    ///
    /// * `reference_point` - The point that will be mapped to (1,1)
    ///
    /// # Returns
    ///
    /// A new Scene with all points transformed
    pub fn normalize(&self, reference_point: Point) -> Self {
        let points = self.points.iter().map(|point| Point {
            x: point.x / reference_point.x,
            y: point.y / reference_point.y,
        });

        Self {
            points: points.collect(),
        }
    }
}
