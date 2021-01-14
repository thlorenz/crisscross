use std::convert::TryInto;

use crate::util::round;

use super::{SignedTilePosition, TilePosition};

const WORLD_POSITION_PRECISION: usize = 8;

#[derive(Clone, Debug, PartialEq)]
pub struct WorldCoords {
    x: f32,
    y: f32,
    #[allow(dead_code)]
    tile_size: f32,
}

impl WorldCoords {
    pub(crate) fn new(x: f32, y: f32, tile_size: f32) -> Self {
        Self {
            x: round(x, WORLD_POSITION_PRECISION),
            y: round(y, WORLD_POSITION_PRECISION),
            tile_size,
        }
    }

    pub fn translated(&self, dx: f32, dy: f32) -> Self {
        Self {
            x: self.x + dx,
            y: self.y + dy,
            tile_size: self.tile_size,
        }
    }

    pub(crate) fn distance(&self, other: &Self) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        dx.hypot(dy)
    }

    #[allow(clippy::cast_precision_loss)]
    pub(crate) fn from_tile_position(tp: &TilePosition, tile_size: f32) -> Self {
        let x = tile_size.mul_add(tp.x as f32, tp.rel_x);
        let y = tile_size.mul_add(tp.y as f32, tp.rel_y);
        Self::new(x, y, tile_size)
    }

    #[allow(clippy::cast_precision_loss)]
    pub(crate) fn from_signed_tile_position(tp: &SignedTilePosition, tile_size: f32) -> Self {
        let x = tile_size.mul_add(tp.x as f32, tp.rel_x);
        let y = tile_size.mul_add(tp.y as f32, tp.rel_y);
        Self::new(x, y, tile_size)
    }

    #[allow(dead_code)]
    pub(crate) fn to_tile_position(&self) -> Result<TilePosition, String> {
        self.to_signed_tile_position().try_into()
    }

    #[allow(clippy::cast_possible_truncation)]
    pub(crate) fn to_signed_tile_position(&self) -> SignedTilePosition {
        let x = (self.x / self.tile_size).trunc() as i64;
        let y = (self.y / self.tile_size).trunc() as i64;
        let rel_x = self.x % self.tile_size;
        let rel_y = self.y % self.tile_size;
        SignedTilePosition::new(x, y, rel_x, rel_y)
    }
}

#[cfg(test)]
mod tests {
    use crate::util::round_stp;

    use super::*;

    #[test]
    fn conversions() {
        let wc = WorldCoords::new(-1.732, 1.0, 1.0);
        let stp = wc.to_signed_tile_position();
        assert_eq!(round_stp(&stp), ((-1, -0.732), (1, 0.0)).into());

        let wc = WorldCoords::new(5.196, 3.0, 1.0);
        let stp = wc.to_signed_tile_position();
        assert_eq!(round_stp(&stp), ((5, 0.196), (3, 0.0)).into());
    }

    #[test]
    fn distance() {
        let tile_size = 1.0;
        let test_cases: Vec<((f32, f32), (f32, f32), f32)> = vec![
            ((0.0, 0.0), (1.0, 0.0), 1.000),
            ((0.0, 0.2), (4.0, 0.8), 4.045),
            ((1.0, 0.2), (-4.0, 0.8), 5.036),
            ((-1.0, 0.2), (4.0, 0.8), 5.036),
        ];
        for ((x1, y1), (x2, y2), distance) in test_cases {
            let wc1 = WorldCoords::new(x1, y1, tile_size);
            let wc2 = WorldCoords::new(x2, y2, tile_size);
            assert_eq!(round(wc1.distance(&wc2), 3), distance);
        }
    }
}
