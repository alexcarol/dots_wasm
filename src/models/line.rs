use models::Dot;
use std::hash::Hash;
use std::hash::Hasher;

#[derive(Copy, Clone, Debug)]
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
impl PartialEq for Line {
    fn eq(&self, other: &'_ Line) -> bool {
        self.a == other.a && self.b == other.b
            || self.a == other.b && self.b == other.a
    }
}
impl Eq for Line {

}

impl Hash for Line {
    fn hash<H: Hasher>(&self, state: &'_ mut H) {
        if self.a.i > self.b.i || self.a.i == self.b.i && self.a.j > self.b.j {
            self.a.hash(state);
            self.b.hash(state);
        } else {
            self.b.hash(state);
            self.a.hash(state);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lines_equal_same_order() {
        assert_eq!(
            Line::new(Dot::new(1, 1), Dot::new(0, 1)),
            Line::new(Dot::new(1, 1), Dot::new(0, 1))
        );
    }

    #[test]
    fn lines_equal_different_order() {
        assert_eq!(
            Line::new(Dot::new(1, 1), Dot::new(0, 1)),
            Line::new(Dot::new(0, 1), Dot::new(1, 1))
        );
    }
}
