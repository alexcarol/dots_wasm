use geometry::Size;
use models::Dot;

/// A model that contains the other models and renders them
pub struct World {
    pub dots: Vec<Vec<Dot>>,
    pub size: Size,
}

impl World {
    /// Returns a new world of the given size
    pub fn new(size: Size) -> World {
        World {
            dots: World::dots(),
            size: size,
        }
    }

    pub fn dots() -> Vec<Vec<Dot>> {
        let mut rows = vec![];
        for i in 1..30 {
            let mut row =  vec![];
            for j in 1..30 {
                row.push( Dot::new(i * 100, j * 100) );
            }
            rows.push(row);
        }

        rows
    }
}


