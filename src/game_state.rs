use geometry::{Size};
use models::World;
use models::Dot;
use models::Mouse;

#[derive(Copy, Clone)]
pub struct PixelPoint {
    pub x: i32,
    pub y: i32,
}


/// The data structure that contains the state of the game
pub struct GameState {
    /// The world contains everything that needs to be drawn
    pub world: World,

    pub current_line_active: bool,
    pub start_dot: Dot,
    pub mouse: Mouse,
}

impl GameState {
    /// Returns a new `GameState` containing a `World` of the given `Size`
    pub fn new(size: Size) -> GameState {
        GameState {
            world: World::new(size),
            current_line_active: false,
            start_dot: Dot::new(0, 0),
            mouse: Mouse::new(0.0, 0.0),
        }
    }

    pub fn on_mouse_up(&mut self) {
        self.current_line_active = false;

        self.world.on_mouse_up(self.start_dot, &self.mouse);

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
