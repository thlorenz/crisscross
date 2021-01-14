#![allow(unused)]
use std::f32::EPSILON;

// work around cargo bug
use crate::{position::SignedTilePosition, TilePosition};

pub fn floats_equal(f1: f32, f2: f32) -> bool {
    (f1 - f2).abs() < EPSILON
}

#[allow(
    clippy::as_conversions,
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation
)]
pub fn round(n: f32, decimals: usize) -> f32 {
    let factor = 10_u64.pow(decimals as u32) as f64;
    ((f64::from(n) * factor).round() / factor) as f32
}

pub fn round_tp(tp: &TilePosition) -> TilePosition {
    let TilePosition { x, y, rel_x, rel_y } = tp;
    TilePosition {
        x: *x,
        y: *y,
        rel_x: round(*rel_x, 3),
        rel_y: round(*rel_y, 3),
    }
}

#[cfg(test)]
pub(crate) fn round_otp(tp: Option<TilePosition>) -> Option<TilePosition> {
    let TilePosition { x, y, rel_x, rel_y } = tp?;
    Some(TilePosition {
        x,
        y,
        rel_x: round(rel_x, 3),
        rel_y: round(rel_y, 3),
    })
}

pub fn round_stp(stp: &SignedTilePosition) -> SignedTilePosition {
    let SignedTilePosition { x, y, rel_x, rel_y } = stp;
    SignedTilePosition {
        x: *x,
        y: *y,
        rel_x: round(*rel_x, 3),
        rel_y: round(*rel_y, 3),
    }
}

#[cfg(test)]
pub(crate) fn round_ostp(tp: Option<SignedTilePosition>) -> Option<SignedTilePosition> {
    let SignedTilePosition { x, y, rel_x, rel_y } = tp?;
    Some(SignedTilePosition {
        x,
        y,
        rel_x: round(rel_x, 3),
        rel_y: round(rel_y, 3),
    })
}
