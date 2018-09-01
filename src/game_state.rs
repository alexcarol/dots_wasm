use pcg_rand::Pcg32Basic;
use rand::SeedableRng;

use geometry::{Position, Size};
use models::World;

pub struct PixelPoint {
    pub x: i32,
    pub y: i32,
}
pub struct Line {
    pub a: PixelPoint,
    pub b: PixelPoint,
}

/// The data structure that contains the state of the game
pub struct GameState {
    /// The world contains everything that needs to be drawn
    pub world: World,
    /// The current score of the player
    pub score: u32,

    pub current_line_active: bool,
    pub current_line: Line,
}

impl GameState {
    /// Returns a new `GameState` containing a `World` of the given `Size`
    pub fn new(size: Size) -> GameState {
        let mut rng = Pcg32Basic::from_seed([42, 42]);
        GameState {
            world: World::new(&mut rng, size),
            score: 0,
            current_line_active: false,
            current_line: Line {
                a: PixelPoint { x: 0, y: 0},
                b: PixelPoint { x:  0, y: 0},
            },
        }
    }

    /// Reset our game-state
    pub fn reset(&mut self) {
        let mut rng = Pcg32Basic::from_seed([42, 42]);

        // Reset score
        self.score = 0;

        // Remove all enemies and bullets
        self.world.enemies.clear();
        self.world.enemies = World::enemies();
        self.current_line =   Line {
            a: PixelPoint { x: 0, y: 0},
            b: PixelPoint { x:  0, y: 0},
        };
        self.current_line_active = false;
    }
}
