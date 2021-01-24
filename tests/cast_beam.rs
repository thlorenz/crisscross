mod common;
use common::round_beam_intersect;
use crisscross::{BeamIntersect, Grid, TilePosition, TileRaycaster};

fn cast(grid: &Grid, center: &TilePosition, width: f32, angle: f32) -> Vec<BeamIntersect> {
    let tc = TileRaycaster::new(grid.clone());
    let bis: Vec<BeamIntersect> = tc
        .cast_beam(&center, width, angle)
        .map(round_beam_intersect)
        .collect();

    #[cfg(feature = "plot")]
    {
        use crisscross::plot::{plot_beam, PlotType};
        plot_beam(&grid, center, width, &angle.into(), &bis, PlotType::File);
    }

    bis
}

#[test]
fn cast_beam_4x4grid() {
    let grid = Grid::new(4, 4, 1.0);
    let center = TilePosition::from(((1, 0.5), (1, 0.5)));
    let width = 0.8;
    let angle = 0.0;
    assert_eq!(
        cast(&grid, &center, width, angle),
        [
            BeamIntersect(0, ((2, 0.000), (1, 0.900)).into()),
            BeamIntersect(0, ((3, 0.000), (1, 0.900)).into())
        ],
    );

    let center = TilePosition::from(((0, 0.0), (0, 0.0)));
    assert_eq!(
        cast(&grid, &center, width, angle),
        [
            BeamIntersect(0, ((1, 0.000), (0, 0.400)).into()),
            BeamIntersect(0, ((2, 0.000), (0, 0.400)).into()),
            BeamIntersect(0, ((3, 0.000), (0, 0.400)).into())
        ],
    );

    let center = TilePosition::from(((0, 0.3), (2, 0.3)));
    let width = 2.2;
    let angle = 320_f32.to_radians();
    assert_eq!(
        cast(&grid, &center, width, angle),
        [
            BeamIntersect(0, ((0, 0.087), (1, 1.000)).into()),
            BeamIntersect(4, ((1, 0.177), (2, 1.000)).into()),
            BeamIntersect(2, ((1, 0.228), (1, 1.000)).into()),
            BeamIntersect(4, ((2, 0.000), (2, 0.309)).into()),
            BeamIntersect(0, ((1, 0.279), (0, 1.000)).into()),
            BeamIntersect(3, ((2, 0.000), (1, 0.831)).into()),
            BeamIntersect(1, ((2, 0.000), (0, 0.874)).into()),
            BeamIntersect(4, ((3, 0.000), (1, 0.470)).into()),
            BeamIntersect(3, ((3, 0.000), (0, 0.992)).into())
        ],
    );

    let center = TilePosition::from(((3, 0.3), (3, 0.3)));
    assert_eq!(
        cast(&grid, &center, width, angle),
        [
            BeamIntersect(2, ((3, 0.087), (2, 1.000)).into()),
            BeamIntersect(0, ((3, 0.138), (1, 1.000)).into())
        ],
    );

    let center = TilePosition::from(((3, 0.3), (1, 0.5)));
    let angle = 135_f32.to_radians();
    assert_eq!(
        cast(&grid, &center, width, angle),
        [
            BeamIntersect(1, ((2, 0.763), (1, 0.000)).into()),
            BeamIntersect(4, ((3, 0.319), (2, 0.000)).into()),
            BeamIntersect(3, ((2, 0.800), (2, 0.000)).into()),
            BeamIntersect(0, ((1, 1.000), (1, 0.244)).into()),
            BeamIntersect(5, ((2, 0.837), (3, 0.000)).into()),
            BeamIntersect(1, ((1, 0.763), (2, 0.000)).into()),
            BeamIntersect(3, ((1, 0.800), (3, 0.000)).into()),
            BeamIntersect(0, ((0, 1.000), (2, 0.244)).into()),
            BeamIntersect(1, ((0, 0.763), (3, 0.000)).into())
        ],
    );
}
