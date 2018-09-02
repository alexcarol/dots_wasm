use geometry::Point;

#[derive(Copy, Clone)]
pub struct Dot {
    point: Point,
    used: bool,
}

impl Dot {
    pub fn new(x: i32, y: i32) -> Dot
    {
        Dot {
            point: Point {
                x: x as f64,
                y: y as f64,
            },
            used: false,
        }
    }
}