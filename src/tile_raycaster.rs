use crate::{
    grid::Grid, intersections::Intersections, intersections_iter::IntersectionsIter,
    position::TilePosition,
};

pub struct TileRaycaster {
    grid: Grid,
}

impl TileRaycaster {
    #[must_use]
    pub const fn new(grid: Grid) -> Self {
        Self { grid }
    }

    #[must_use]
    pub fn tiles_in_path(&self, tp: TilePosition, angle: f32) -> IntersectionsIter {
        let intersections = Intersections::new(self.grid.clone(), tp, angle);
        intersections.into_iter()
    }

    pub fn last_valid<P>(&self, tp: TilePosition, angle: f32, is_valid: P) -> Option<TilePosition>
    where
        P: FnMut(&TilePosition) -> bool,
    {
        self.tiles_in_path(tp, angle).take_while(is_valid).last()
    }
}
