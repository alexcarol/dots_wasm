use geometry::{Size};
use models::World;
use models::Dot;
use models::Mouse;


#[derive(Copy, Clone)]
pub struct PixelPoint {
    pub x: i32,
    pub y: i32,
}

#[derive(Copy, Clone)]
pub struct Line {
    pub a: Dot,
    pub b: Dot,
}

impl Line {
    pub fn new(a: Dot, b: Dot) -> Line {
        Line {
            a,
            b,
        }
    }
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
        GameState {
            world: World::new(size),
            score: 0,
            current_line_active: false,
            current_line: Line {
                a: Dot::new(0.0, 0.0, 0, 0),
                b: Dot::new(0.0, 0.0, 0, 0),
            },
        }
    }

    pub fn on_mouse_up(&mut self) {
        self.current_line_active = false;

        let mouse = Mouse::new(self.current_line.b.point.x, self.current_line.b.point.y);

        self.world.on_mouse_up(self.current_line.a, &mouse);

    }

/*
never used???
/// Reset our game-state
    pub fn reset(&mut self) {
        // Reset score
        self.score = 0;

        // Remove all enemies and bullets
        self.world.dots = World::dots();
        self.current_line = Line {
            a: Dot::new(0.0, 0.0, 0, 0),
            b: Dot::new(0.0, 0.0, 0, 0),
        };
        self.current_line_active = false;
    }*/
}
