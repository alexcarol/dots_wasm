use geometry::Point;
use models::Mouse;
use std::hash::Hash;
use std::hash::Hasher;

#[derive(Copy, Clone, PartialEq)]
pub struct Dot {
    pub point: Point,
    pub i: usize,
    pub j: usize,
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
        }
    }

    pub fn collides_with(&self, mouse: &Mouse) -> bool {
        self.point.intersect_circle(&mouse.point, 10.0)
    }
}

impl Hash for Dot {
    fn hash<H: Hasher>(&self, state: &'_ mut H) {
        // TODO this assumes i and j will always be consistent
        self.i.hash(state);
        self.j.hash(state);
    }
}