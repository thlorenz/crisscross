mod common;
use common::round_beam_intersect;
use crisscross::{AngleRad, BeamIntersect, Grid, TilePosition, TileRaycaster};

fn cast_beam_for_angle<T: Into<AngleRad>>(
    tc: &TileRaycaster,
    center: &TilePosition,
    width: f32,
    angle: T,
) -> Vec<BeamIntersect> {
    tc.cast_beam(&center, width, angle)
        .map(round_beam_intersect)
        .collect()
}

#[test]
fn cast_beam_4x4grid() {
    let tc = TileRaycaster::new(Grid::new(4, 4, 1.0));
    let center = TilePosition::from(((1, 0.5), (1, 0.5)));
    let width = 0.8;
    let angle = 0.0;
    assert_eq!(
        cast_beam_for_angle(&tc, &center, width, angle),
        [
            BeamIntersect(0, ((2, 0.000), (1, 0.900)).into()),
            BeamIntersect(0, ((3, 0.000), (1, 0.900)).into())
        ],
    );

    /*
    let center = TilePosition::from(((0, 0.0), (0, 0.0)));
    assert_eq!(
        cast_beam_for_angle(&tc, &center, width, angle),
        [
            BeamIntersect(0, ((1, 0.000), (0, 0.400)).into()),
            BeamIntersect(2, ((0, 1.000), (0, -0.400)).into()),
            BeamIntersect(0, ((2, 0.000), (0, 0.400)).into()),
            BeamIntersect(0, ((3, 0.000), (0, 0.400)).into())
        ],
    );
    */
}
