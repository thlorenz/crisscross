/*
mod common;
use common::round_beam_intersect;
use crisscross::{BeamIntersect, Grid, TilePosition, TileRaycaster};

#[test]
fn cast_beam_0deg() {
    todo!()
    let tc = TileRaycaster::new(Grid::new(4, 4, 1.0));
    let center: TilePosition = ((1, 0.5), (1, 0.5)).into();
    let width = 0.8;
    assert_eq!(
        tc.cast_beam(&center, width, 0_f32.to_radians())
            .map(round_beam_intersect)
            .collect::<Vec<BeamIntersect>>(),
        [
            BeamIntersect(0, ((2, 0.000), (1, 0.900)).into()),
            BeamIntersect(0, ((3, 0.000), (1, 0.900)).into())
        ],
    );

    assert_eq!(
        tc.cast_ray(((0, 0.0), (0, 0.0)).into(), 0_f32.to_radians())
            .map(round_tp)
            .collect::<Vec<TilePosition>>(),
        [
            ((1, 0.000), (0, 0.000)).into(),
            ((2, 0.000), (0, 0.000)).into(),
            ((3, 0.000), (0, 0.000)).into()
        ],
    );
}
*/
