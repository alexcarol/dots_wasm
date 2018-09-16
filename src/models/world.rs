use geometry::Size;
use models::Dot;
use models::Mouse;
use std::collections::HashSet;
use models::Line;

#[derive(PartialEq)]
pub enum ActivePlayer {
    A,
    B,
}

impl ActivePlayer {
    pub fn is_a(&self) -> bool {
        *self == ActivePlayer::A
    }
}

/// A model that contains the other models and renders them
pub struct World {
    pub dots: Vec<Vec<Dot>>,
    pub size: Size,
    pub lines_a: HashSet<Line>,
    pub lines_b: HashSet<Line>,
    pub score_a: u32,
    pub score_b: u32,
    pub active_player: ActivePlayer,
}

impl World {
    /// Returns a new world of the given size
    pub fn new(size: Size) -> World {
        World {
            dots: World::dots(size),
            lines_a: HashSet::new(),
            lines_b: HashSet::new(),
            score_a: 0,
            score_b: 0,
            active_player: ActivePlayer::A,
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
        let end_dot = *collision.unwrap();

        if start_dot.is_contiguous(end_dot) {
            let line = Line::new(start_dot, end_dot);

            if self.active_player == ActivePlayer::A {
                if !self.lines_b.contains(&line) && self.lines_a.insert(line) {

                    if self.scored(line) {
                        self.score_a += 1;
                    } else {
                        self.active_player = ActivePlayer::B;
                    }

                }
            } else {
                if !self.lines_a.contains(&line) && self.lines_b.insert(line) {
                    if self.scored(line) {
                        self.score_b += 1;
                    } else {
                        self.active_player = ActivePlayer::A;
                    }
                }
            }
        }
    }

    fn scored(&self, line: Line) -> bool {
        let squares =  if line.a.i == line.b.i {
            let i = line.a.i;
            vec![
                World::square(line, Dot::new(i + 1, line.a.j), Dot::new(i + 1, line.b.j)),
                World::square(line, Dot::new(i - 1, line.a.j), Dot::new(i - 1, line.b.j)),
            ]
        } else {
            let j = line.a.j;
            vec![
                World::square(line, Dot::new(line.a.i, j + 1), Dot::new(line.b.i, j + 1)),
                World::square(line, Dot::new(line.a.i, j - 1), Dot::new(line.b.i,j - 1)),
            ]
        };

        squares.iter().any(
            |square| square.iter().all(
                |line| self.lines_a.contains(line) || self.lines_b.contains(line)
            )
        )
    }

    fn square(line: Line, dot_c: Dot, dot_d: Dot) -> Vec<Line> {
        vec![
            Line::new(
                line.a,
                dot_c,
            ),
            Line::new(
                line.b,
                dot_d,
            ),
            Line::new(
                dot_c,
                dot_d,
            ),
        ]
    }

    pub fn dots(_size: Size) -> Vec<Vec<Dot>> {
        let mut rows = vec![];
        for i in 0..3 {
            let mut row = vec![];
            for j in 0..3 {
                row.push(Dot::new(i, j));
            }
            rows.push(row);
        }

        rows
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn score_is_added() {
        let mut world = World::new(Size::new(0.0, 0.0));
        world.lines_a =  vec![
            Line::new(Dot::new(0, 0), Dot::new(0, 1)),
            Line::new(Dot::new(0, 0), Dot::new(1, 0)),
            Line::new(Dot::new(1, 0), Dot::new(1, 1)),
        ].iter().cloned().collect();

        assert!(world.scored(Line::new(Dot::new(1, 1), Dot::new(0, 1))), "counts scored");
    }
}

