mod common;
use common::{round_cutoff, round_tp};
use crisscross::{Crossing, Grid, TilePosition, TileRaycaster};

#[test]
fn last_valid() {
    let tc = TileRaycaster::new(Grid::new(4, 4, 1.0));

    assert_eq!(
        tc.last_valid(((0, 0.0), (0, 0.0)).into(), 30_f32.to_radians(), |tp| {
            tp.y < 2
        })
        .map(round_tp),
        Some(((3, 0.000), (1, 0.732)).into(),),
    );
}

#[test]
fn cutoff() {
    let tc = TileRaycaster::new(Grid::new(4, 4, 1.0));
    let cutoff = round_cutoff(tc.crossing(
        ((0, 0.0), (0, 0.0)).into(),
        30_f32.to_radians(),
        |tp| tp.y < 2,
    ));

    assert_eq!(
        cutoff,
        Crossing {
            valid: Some(((3, 0.000), (1, 0.732)).into(),),
            invalid: Some(((3, 0.464), (2, 0.000)).into()),
        }
    );

    let tp_0_0: TilePosition = ((0, 0.0), (0, 0.0)).into();
    assert_eq!(
        round_cutoff(tc.crossing(tp_0_0.clone(), 0_f32.to_radians(), |tp| tp.x <= 0)),
        Crossing::default(),
    );
    assert_eq!(
        round_cutoff(tc.crossing(tp_0_0.clone(), 0_f32.to_radians(), |tp| tp.x <= 1)),
        Crossing {
            valid: Some(((1, 0.000), (0, 0.000)).into()),
            invalid: Some(((2, 0.000), (0, 0.000)).into())
        }
    );
    assert_eq!(
        round_cutoff(tc.crossing(tp_0_0.clone(), 0_f32.to_radians(), |tp| tp.x <= 2)),
        Crossing {
            valid: Some(((2, 0.000), (0, 0.000)).into()),
            invalid: Some(((3, 0.000), (0, 0.000)).into())
        }
    );
    assert_eq!(
        round_cutoff(tc.crossing(tp_0_0.clone(), 0_f32.to_radians(), |tp| tp.x <= 3)),
        Crossing {
            valid: Some(((3, 0.000), (0, 0.000)).into()),
            invalid: None,
        }
    );
    assert_eq!(
        round_cutoff(tc.crossing(tp_0_0.clone(), 0_f32.to_radians(), |tp| tp.x <= 4)),
        Crossing {
            valid: Some(((3, 0.000), (0, 0.000)).into()),
            invalid: None,
        }
    );
}
