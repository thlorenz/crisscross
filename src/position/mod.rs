#![allow(clippy::clippy::as_conversions)]
// assert_eq!(u32::MAX, 4294967295)
// assert_eq!(i64::MAX, 9223372036854775807)
// pub const MAX: f32 = 3.40282347e+38_f32

mod tile_position;
mod world_coords;

pub use tile_position::*;
pub use world_coords::*;
