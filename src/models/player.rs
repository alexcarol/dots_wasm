use rand::Rng;

use geometry::{Point, Size};
use super::Vector;
use geometry::{Advance, Collide, Position};

/// The `Player` is the rocket controlled by the user
#[derive(Default)]
pub struct Player {
    pub vector: Vector
}

derive_position_direction!(Player);

impl Player {

}

impl Collide for Player {
    fn radius(&self) -> f64 { 6.0 }
}
