use geometry::Size;
use models::Dot;
use models::Mouse;
use game_state::Line;

/// A model that contains the other models and renders them
pub struct World {
    pub dots: Vec<Vec<Dot>>,
    pub size: Size,
    pub lines: Vec<Line>,
}

impl World {
    /// Returns a new world of the given size
    pub fn new(size: Size) -> World {
        World {
            dots: World::dots(),
            lines: vec![],
            size: size,
        }
    }

    pub fn on_mouse_up(&mut self, start_dot: Dot, mouse: &Mouse) {
        let collision = self
            .dots
            .iter()
            .filter_map(|row| row.iter().filter_map(|dot| if dot.collides_with(mouse) {
                Some(dot)
            } else {
                None
            }).next()).next()
        ;


        if !collision.is_some() {
            return;
        }

        self.lines.push(Line::new(start_dot, *collision.unwrap()))
    }

    pub fn dots() -> Vec<Vec<Dot>> {
        let mut rows = vec![];
        for i in 1..30 {
            let mut row = vec![];
            for j in 1..30 {
                row.push(Dot::new((i * 100) as f64, (j * 100) as f64, i - 1, j - 1));
            }
            rows.push(row);
        }

        rows
    }
}


