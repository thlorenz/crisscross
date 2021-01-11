use crate::{
    grid::Grid, intersections::Intersections, intersections_iter::IntersectionsIter,
    position::TilePosition,
};

pub struct TileRaycaster {
    grid: Grid,
    blocked_tiles: Option<Vec<(u32, u32)>>,
}

impl TileRaycaster {
    fn new(grid: Grid) -> Self {
        Self {
            grid,
            blocked_tiles: None,
        }
    }

    pub fn all_tiles(&self, tp: TilePosition, angle: f32) -> IntersectionsIter {
        let intersections = Intersections::new(self.grid.clone(), tp, angle);
        intersections.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grid_4x4_0deg() {
        let tc = TileRaycaster::new(Grid::new(4, 4, 1.0));
        assert_eq!(
            tc.all_tiles(((1, 0.5), (1, 0.5)).into(), 0_f32.to_radians())
                .collect::<Vec<TilePosition>>(),
            [
                ((2, 0.000), (1, 0.500)).into(),
                ((3, 0.000), (1, 0.500)).into()
            ],
        );

        assert_eq!(
            tc.all_tiles(((0, 0.0), (0, 0.0)).into(), 0_f32.to_radians())
                .collect::<Vec<TilePosition>>(),
            [
                ((1, 0.000), (0, 0.000)).into(),
                ((2, 0.000), (0, 0.000)).into(),
                ((3, 0.000), (0, 0.000)).into()
            ],
        );
    }

    #[test]
    fn grid_4x4_30deg() {
        let tc = TileRaycaster::new(Grid::new(4, 4, 1.0));
        assert_eq!(
            tc.all_tiles(((1, 0.5), (1, 0.5)).into(), 30_f32.to_radians())
                .collect::<Vec<TilePosition>>(),
            [
                ((2, 0.000), (1, 0.789)).into(),
                ((2, 0.366), (2, 0.000)).into(),
                ((3, 0.000), (2, 0.366)).into(),
            ]
        );

        assert_eq!(
            tc.all_tiles(((0, 0.0), (0, 0.0)).into(), 30_f32.to_radians())
                .collect::<Vec<TilePosition>>(),
            [
                ((1, 0.000), (0, 0.577)).into(),
                ((1, 0.732), (1, 0.000)).into(),
                ((2, 0.000), (1, 0.154)).into(),
                ((3, 0.000), (1, 0.731)).into(),
                ((3, 0.464), (2, 0.000)).into(),
            ]
        );
    }

    #[test]
    fn grid_4x4_45deg() {
        let tc = TileRaycaster::new(Grid::new(4, 4, 1.0));
        assert_eq!(
            tc.all_tiles(((1, 0.5), (1, 0.5)).into(), 45_f32.to_radians())
                .collect::<Vec<TilePosition>>(),
            [
                ((2, 0.000), (2, 0.000)).into(),
                ((3, 0.000), (3, 0.000)).into()
            ]
        );
    }

    #[test]
    fn grid_4x4_60deg() {
        let tc = TileRaycaster::new(Grid::new(4, 4, 1.0));
        assert_eq!(
            tc.all_tiles(((1, 0.5), (1, 0.5)).into(), 60_f32.to_radians())
                .collect::<Vec<TilePosition>>(),
            [
                ((1, 0.789), (2, 0.000)).into(),
                ((2, 0.000), (2, 0.366)).into(),
                ((2, 0.366), (3, 0.000)).into()
            ]
        );
    }

    #[test]
    fn grid_4x4_90deg() {
        let tc = TileRaycaster::new(Grid::new(4, 4, 1.0));
        assert_eq!(
            tc.all_tiles(((1, 0.5), (1, 0.5)).into(), 90_f32.to_radians())
                .collect::<Vec<TilePosition>>(),
            [
                ((1, 0.500), (2, 0.000)).into(),
                ((1, 0.500), (3, 0.000)).into()
            ]
        );
    }

    #[test]
    fn grid_4x4_150deg() {
        let tc = TileRaycaster::new(Grid::new(4, 4, 1.0));
        assert_eq!(
            tc.all_tiles(((1, 0.5), (1, 0.5)).into(), 150_f32.to_radians())
                .collect::<Vec<TilePosition>>(),
            [
                ((0, 1.000), (1, 0.789)).into(),
                ((0, 0.634), (2, 0.000)).into()
            ]
        );
    }

    #[test]
    fn grid_4x4_210deg() {
        let tc = TileRaycaster::new(Grid::new(4, 4, 1.0));
        assert_eq!(
            tc.all_tiles(((1, 0.5), (1, 0.5)).into(), 210_f32.to_radians())
                .collect::<Vec<TilePosition>>(),
            [
                ((0, 1.000), (1, 0.211)).into(),
                ((0, 0.634), (0, 1.000)).into()
            ]
        );
    }

    #[test]
    fn grid_4x4_330deg() {
        let tc = TileRaycaster::new(Grid::new(4, 4, 1.0));
        assert_eq!(
            tc.all_tiles(((1, 0.5), (1, 0.5)).into(), 330_f32.to_radians())
                .collect::<Vec<TilePosition>>(),
            [
                ((2, 0.000), (1, 0.211)).into(),
                ((2, 0.366), (0, 1.000)).into(),
                ((3, 0.000), (0, 0.634)).into()
            ]
        );

        assert_eq!(
            tc.all_tiles(((0, 0.25), (3, 0.25)).into(), 330_f32.to_radians())
                .collect::<Vec<TilePosition>>(),
            [
                ((0, 0.683), (2, 1.000)).into(),
                ((1, 0.000), (2, 0.817)).into(),
                ((2, 0.000), (2, 0.240)).into(),
                ((2, 0.415), (1, 1.000)).into(),
                ((3, 0.000), (1, 0.663)).into(),
            ]
        );
    }
}
