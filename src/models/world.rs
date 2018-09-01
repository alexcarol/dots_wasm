use rand::Rng;

use geometry::Size;
use models::{Enemy, Particle};
use models::Vector;
use geometry::Point;
use pcg_rand::Pcg32Basic;

/// A model that contains the other models and renders them
pub struct World {
    pub particles: Vec<Particle>,
    pub enemies: Vec<Enemy>,
    pub size: Size,
}

impl World {
    /// Returns a new world of the given size
    pub fn new<R: Rng>(rng: &mut R, size: Size) -> World {
        World {
            particles: vec![],
            enemies: World::enemies(),
            size: size,
        }
    }

    pub fn enemies() -> Vec<Enemy> {
        let mut point_vec = vec![];
        for i in 1..30 {
            for j in 1..30 {
                point_vec.push(Point { x: i as f64 * 100.0, y: j as f64 * 100.0 });
            }
        }

        point_vec.iter().map(|point| Enemy::new(Vector::new(*point, 0.0))).collect()
    }
}


