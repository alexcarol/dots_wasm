use geometry::Point;
use geometry::{Collide};
use geometry::Position;

pub struct Mouse {
    pub point: Point,
}

impl Mouse{
    /// Create a enemy with the given vector
    pub fn new(x: f64, y: f64) -> Mouse {
        Mouse {
            point: Point::new(x, y)
        }
    }
}

impl Collide for Mouse {
    fn radius(&self) -> f64 { 10.0 }
}

impl Position for Mouse {
    fn x(&self) -> f64 {
        self.point.x
    }

    fn x_mut(&mut self) -> &'_ mut f64 {
        &mut self.point.x
    }

    fn y(&self) -> f64 {
        self.point.y
    }

    fn y_mut(&mut self) -> &'_ mut f64 {
        &mut self.point.y
    }

    fn position(&self) -> Point {
        self.point
    }
}