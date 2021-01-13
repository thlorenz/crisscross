mod common;
use common::round_tp;
use crisscross::{Grid, TileRaycaster};

#[test]
fn grid_4x4_30deg() {
    let tc = TileRaycaster::new(Grid::new(4, 4, 1.0));
    assert_eq!(
        tc.last_valid(((0, 0.0), (0, 0.0)).into(), 30_f32.to_radians(), |tp| {
            tp.y < 2
        })
        .map(round_tp),
        Some(((3, 0.000), (1, 0.732)).into(),),
    );
}
