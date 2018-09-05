use geometry::Point;
use models::Mouse;

#[derive(Copy, Clone)]
pub struct Dot {
    pub point: Point,
    pub i: usize,
    pub j: usize,
    used: bool,
}

impl Dot {
    pub fn new(x: f64, y: f64, i: usize, j: usize) -> Dot
    {
        Dot {
            point: Point {
                x: x as f64,
                y: y as f64,
            },
            i: i,
            j: j,
            used: false,
        }
    }

    pub fn mark_used(&mut self) {
        self.used = true;
    }

    pub fn is_used(&self) -> bool {
        self.used
    }

    pub fn collides_with(&self, mouse: &Mouse) -> bool {
        self.point.intersect_circle(&mouse.point, 10.0)
    }
}