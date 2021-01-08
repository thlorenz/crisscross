use std::convert::TryInto;

use crate::util::round;

use super::{SignedTilePosition, TilePosition};

const WORLD_POSITION_PRECISION: usize = if cfg!(test) { 3 } else { 8 };

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
        WorldCoords {
            x: self.x + dx,
            y: self.y + dy,
            tile_size: self.tile_size,
        }
    }
    pub(crate) fn from_tile_position(tp: &TilePosition, tile_size: f32) -> Self {
        let x = (tile_size * tp.x as f32) + tp.rel_x;
        let y = (tile_size * tp.y as f32) + tp.rel_y;
        WorldCoords::new(x, y, tile_size)
    }

    #[allow(dead_code)]
    pub(crate) fn to_tile_position(&self) -> Result<TilePosition, String> {
        self.to_signed_tile_position().try_into()
    }

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
    use super::*;

    #[test]
    fn conversions() {
        let wc = WorldCoords::new(-1.732, 1.0, 1.0);
        let stp = wc.to_signed_tile_position();
        assert_eq!(stp, ((-1, -0.732), (1, 0.0)).into());
    }
}
