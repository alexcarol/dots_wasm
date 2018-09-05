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

    pub fn on_mouse_up(&mut self, start_dot: &Dot, mouse: &Mouse) {
        let end_dot =  {
            // this works, but using this at the end doesn't, wtf
            let collision = self
                .dots
                .iter_mut()
                .filter_map(|row| row.iter_mut().filter_map(|dot| if dot.collides_with(mouse) {
                    Some(dot)
                } else {
                    None
                }).next()).next()
            ;


            if !collision.is_some() {
                return;
            }

            let mut_end_dot = collision.unwrap();
            mut_end_dot.mark_used();

            *mut_end_dot
        };

        let mut_start_dot = &mut self.dots[start_dot.i][start_dot.j];

        mut_start_dot.mark_used();

        self.lines.push(Line::new(*mut_start_dot, end_dot))
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


