use geometry::{Size};
use models::World;
use models::Dot;

#[derive(Copy, Clone)]
pub struct PixelPoint {
    pub x: i32,
    pub y: i32,
}

#[derive(Copy, Clone)]
pub struct Line<'a> {
    pub a: &'a Dot,
    pub b: &'a Dot,
}

/// The data structure that contains the state of the game
pub struct GameState<'a> {
    /// The world contains everything that needs to be drawn
    pub world: World,
    /// The current score of the player
    pub score: u32,

    pub lines: Vec<Line<'a>>,

    pub current_line_active: bool,
    pub current_line: Line<'a>,
}

impl<'a> GameState<'a> {
    /// Returns a new `GameState` containing a `World` of the given `Size`
    pub fn new(size: Size) -> GameState<'a> {
        GameState {
            world: World::new(size),
            score: 0,
            current_line_active: false,
            current_line: Line {
                a: &Dot::new(0, 0),
                b: &Dot::new(0, 0),
            },
            lines: vec![],
        }
    }

    /// Reset our game-state
    pub fn reset(&mut self) {
        // Reset score
        self.score = 0;

        // Remove all enemies and bullets
        self.world.dots = World::dots();
        self.current_line = Line {
            a: &Dot::new(0, 0),
            b: &Dot::new(0, 0),
        };
        self.current_line_active = false;
    }
}
