use std::fmt;

use crate::util::round;

#[derive(Clone)]
pub struct TilePosition {
    pub x: u32,
    pub y: u32,
    // Offset from Tile Lower Left
    pub rel_x: f32,
    pub rel_y: f32,
}

pub(crate) struct SignedTilePosition {
    pub x: i64,
    pub y: i64,
    pub rel_x: f32,
    pub rel_y: f32,
}

impl SignedTilePosition {
    pub fn new(x: i64, y: i64, rel_x: f32, rel_y: f32) -> Self {
        Self { x, y, rel_x, rel_y }
    }
}

impl From<TilePosition> for SignedTilePosition {
    fn from(tp: TilePosition) -> Self {
        Self {
            x: tp.x as i64,
            y: tp.y as i64,
            rel_x: tp.rel_x,
            rel_y: tp.rel_y,
        }
    }
}

impl From<SignedTilePosition> for Option<TilePosition> {
    fn from(tp: SignedTilePosition) -> Self {
        if tp.x >= 0 && tp.y >= 0 {
            Some(TilePosition {
                x: tp.x as u32,
                y: tp.y as u32,
                rel_x: tp.rel_x,
                rel_y: tp.rel_y,
            })
        } else {
            None
        }
    }
}

const TILE_POSITION_DEBUG_PRECISION: usize = 3;
impl fmt::Debug for TilePosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let p = TILE_POSITION_DEBUG_PRECISION;
        write!(
            f,
            "(({}, {:.*}), ({}, {:.*})).into()",
            self.x, p, self.rel_x, self.y, p, self.rel_y
        )
    }
}

impl PartialEq for TilePosition {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x
            && self.y == other.y
            && round(self.rel_x, TILE_POSITION_DEBUG_PRECISION)
                == round(other.rel_x, TILE_POSITION_DEBUG_PRECISION)
            && round(self.rel_y, TILE_POSITION_DEBUG_PRECISION)
                == round(other.rel_y, TILE_POSITION_DEBUG_PRECISION)
    }
}

impl TilePosition {
    pub fn new(x: u32, y: u32, rel_x: f32, rel_y: f32) -> Self {
        Self { x, y, rel_x, rel_y }
    }
    #[allow(dead_code)]
    pub fn centered(x: u32, y: u32, tile_size: f32) -> Self {
        Self::new(x, y, tile_size / 2.0, tile_size / 2.0)
    }

    pub fn move_to_edge(&self, dx: i32, dy: i32, tile_size: f32) -> Self {
        let rel_x = TilePosition::edge_for_move(dx, tile_size);
        let rel_y = TilePosition::edge_for_move(dy, tile_size);
        Self {
            x: self.x + dx as u32,
            y: self.y + dy as u32,
            rel_x,
            rel_y,
        }
    }

    fn edge_for_move(d: i32, tile_size: f32) -> f32 {
        if d < 0 {
            tile_size
        } else {
            0.0
        }
    }

    #[allow(dead_code)]
    pub(crate) fn empty() -> Self {
        Self::new(0, 0, 0.0, 0.0)
    }
}

#[cfg(test)]
impl From<((u32, f32), (u32, f32))> for TilePosition {
    fn from(((x, rel_x), (y, rel_y)): ((u32, f32), (u32, f32))) -> Self {
        TilePosition::new(x, y, rel_x, rel_y)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorldCoords {
    x: f32,
    y: f32,
    #[allow(dead_code)]
    tile_size: f32,
}

#[allow(dead_code)]
impl WorldCoords {
    fn new(x: f32, y: f32, tile_size: f32) -> Self {
        Self { x, y, tile_size }
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

    pub(crate) fn to_tile_position(&self) -> TilePosition {
        let x = (self.x / self.tile_size).floor() as u32;
        let y = (self.y / self.tile_size).floor() as u32;
        let rel_x = self.x % self.tile_size;
        let rel_y = self.y % self.tile_size;
        TilePosition::new(x, y, rel_x, rel_y)
    }

    pub(crate) fn to_signed_tile_position(&self) -> SignedTilePosition {
        let x = (self.x / self.tile_size).floor() as i64;
        let y = (self.y / self.tile_size).floor() as i64;
        let rel_x = self.x % self.tile_size;
        let rel_y = self.y % self.tile_size;
        SignedTilePosition::new(x, y, rel_x, rel_y)
    }
}
