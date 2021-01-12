use std::{convert::TryFrom, fmt, ops};

use crate::util::round;

use super::WorldCoords;

const TILE_POSITION_PRECISION: usize = if cfg!(test) { 3 } else { 8 };

#[derive(Clone)]
pub struct TilePosition {
    pub x: u32,
    pub y: u32,
    // Offset from Tile Lower Left
    pub rel_x: f32,
    pub rel_y: f32,
}

#[derive(Debug, PartialEq)]
pub struct SignedTilePosition {
    pub x: i64,
    pub y: i64,
    pub rel_x: f32,
    pub rel_y: f32,
}

impl TilePosition {
    #[must_use]
    pub fn new(x: u32, y: u32, rel_x: f32, rel_y: f32) -> Self {
        Self {
            x,
            y,
            rel_x: round(rel_x, TILE_POSITION_PRECISION),
            rel_y: round(rel_y, TILE_POSITION_PRECISION),
        }
    }

    pub fn distance<'a, T>(&self, other: T, tile_size: f32) -> f32
    where
        T: Into<&'a Self>,
    {
        self.to_world_coords(tile_size)
            .distance(&other.into().to_world_coords(tile_size))
    }

    fn to_world_coords(&self, tile_size: f32) -> WorldCoords {
        WorldCoords::from_tile_position(self, tile_size)
    }
}

impl SignedTilePosition {
    pub fn new(x: i64, y: i64, rel_x: f32, rel_y: f32) -> Self {
        Self {
            x,
            y,
            rel_x: round(rel_x, TILE_POSITION_PRECISION),
            rel_y: round(rel_y, TILE_POSITION_PRECISION),
        }
    }

    pub fn normalized(self, tile_size: f32) -> Self {
        let dts = 2.0 * tile_size;
        let Self { x, y, rel_x, rel_y } = self;
        debug_assert!(-dts < rel_x && rel_x < dts);
        debug_assert!(-dts < rel_y && rel_y < dts);

        #[allow(clippy::clippy::cast_precision_loss)]
        // pub const MAX: f32 = 3.40282347e+38_f32
        WorldCoords::new(x as f32 + rel_x, y as f32 + rel_y, tile_size).to_signed_tile_position()
    }
}

impl From<TilePosition> for SignedTilePosition {
    fn from(tp: TilePosition) -> Self {
        Self {
            x: i64::from(tp.x),
            y: i64::from(tp.y),
            rel_x: tp.rel_x,
            rel_y: tp.rel_y,
        }
    }
}

impl TryFrom<SignedTilePosition> for TilePosition {
    type Error = String;
    fn try_from(tp: SignedTilePosition) -> Result<Self, Self::Error> {
        if tp.x >= 0 && tp.y >= 0 {
            #[allow(
                clippy::clippy::cast_sign_loss,
                clippy::clippy::cast_possible_truncation
            )]
            Ok(Self {
                x: tp.x as u32,
                y: tp.y as u32,
                rel_x: tp.rel_x,
                rel_y: tp.rel_y,
            })
        } else {
            Err("Tile Position is off grid, cannot convert".to_string())
        }
    }
}

impl fmt::Debug for TilePosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let p = TILE_POSITION_PRECISION;
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
            && round(self.rel_x, TILE_POSITION_PRECISION)
                == round(other.rel_x, TILE_POSITION_PRECISION)
            && round(self.rel_y, TILE_POSITION_PRECISION)
                == round(other.rel_y, TILE_POSITION_PRECISION)
    }
}

impl ops::Sub<&TilePosition> for &TilePosition {
    type Output = SignedTilePosition;

    #[allow(clippy::integer_arithmetic)]
    fn sub(self, rhs: &TilePosition) -> Self::Output {
        let dx = i64::from(self.x) - i64::from(rhs.x);
        let dy = i64::from(self.y) - i64::from(rhs.y);
        let rel_x = self.rel_x - rhs.rel_x;
        let rel_y = self.rel_y - rhs.rel_y;
        SignedTilePosition::new(dx, dy, rel_x, rel_y)
    }
}

impl ops::Sub<TilePosition> for TilePosition {
    type Output = SignedTilePosition;
    fn sub(self, rhs: Self) -> Self::Output {
        &self - &rhs
    }
}

impl ops::Add<&SignedTilePosition> for &TilePosition {
    type Output = SignedTilePosition;

    #[allow(clippy::integer_arithmetic)]
    fn add(self, rhs: &SignedTilePosition) -> Self::Output {
        let dx = i64::from(self.x) + rhs.x;
        let dy = i64::from(self.y) + rhs.y;
        let rel_x = self.rel_x + rhs.rel_x;
        let rel_y = self.rel_y + rhs.rel_y;
        SignedTilePosition::new(dx, dy, rel_x, rel_y)
    }
}

impl ops::Add<SignedTilePosition> for TilePosition {
    type Output = SignedTilePosition;
    fn add(self, rhs: SignedTilePosition) -> Self::Output {
        &self + &rhs
    }
}

#[cfg(test)]
impl From<((i64, f32), (i64, f32))> for SignedTilePosition {
    fn from(((x, rel_x), (y, rel_y)): ((i64, f32), (i64, f32))) -> Self {
        Self::new(x, y, rel_x, rel_y)
    }
}

impl From<((u32, f32), (u32, f32))> for TilePosition {
    fn from(((x, rel_x), (y, rel_y)): ((u32, f32), (u32, f32))) -> Self {
        Self::new(x, y, rel_x, rel_y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn subtract() {
        #[rustfmt::skip]
        assert_eq!(
                    TilePosition::new(2,  3, 0.0, 0.0)
                  - TilePosition::new(1,  4, 0.0, 0.0),
              SignedTilePosition::new(1, -1, 0.0, 0.0)
        );

        #[rustfmt::skip]
        assert_eq!(
                    TilePosition::new(1,  1,  0.5, 0.2) 
                  - TilePosition::new(1,  4,  1.0, 0.1),
              SignedTilePosition::new(0, -3, -0.5, 0.1)
        );
    }

    #[test]
    fn distance() {
        let tp1: TilePosition = ((1, 0.0), (3, 0.3)).into();
        let tp2: TilePosition = ((4, 0.1), (8, 0.8)).into();
        assert_eq!(round(tp1.distance(&tp2, 1.0), 3), 6.313);
    }
}
