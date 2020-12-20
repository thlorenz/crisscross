// TODO(thlorenz): remove later
#![allow(dead_code)]

use std::f32::consts::{PI, TAU};

use crate::position::{SignedTilePosition, TilePosition, WorldCoords};

const DEG_90: f32 = PI * 0.5;
const DEG_270: f32 = PI * 1.5;

type GridSize = (u32, u32);

/// Assumes origin (0, 0) is at bottom left.
/// Assumes relative tile position are based on (0.0, 0.0) being located at the bottom left of each
/// tile.
pub(crate) struct Intersections {
    grid_size: GridSize,
    tile_size: f32,
    tp: TilePosition,
    wc: WorldCoords,
    angle: f32,
    tan: f32,
    direction_x: DirectionX,
    direction_y: DirectionY,
}

#[derive(Debug, PartialEq)]
enum DirectionX {
    Left,
    Right,
    Parallel,
}
#[derive(Debug, PartialEq)]
enum DirectionY {
    Up,
    Down,
    Parallel,
}

#[derive(Default, Debug)]
struct InitialIntersects {
    x_axis: Option<(WorldCoords, TilePosition)>,
    y_axis: Option<(WorldCoords, TilePosition)>,
}

impl Intersections {
    fn new(grid_size: GridSize, tile_size: f32, tp: TilePosition, angle: f32) -> Self {
        let wc = WorldCoords::from_tile_position(&tp, tile_size);

        // Clamp angle to 0.0..TAU
        let angle = match angle {
            x if x >= TAU => x - TAU,
            x if x < 0.0 => x + TAU,
            x => x,
        };
        let tan = angle.tan();

        let direction_x = match angle {
            x if x == DEG_90 || x == DEG_270 => DirectionX::Parallel,
            x if x < DEG_90 || x > DEG_270 => DirectionX::Right,
            x if x > DEG_90 && x < DEG_270 => DirectionX::Left,
            _ => panic!("Unhandled x direction for angle {}°", angle.to_degrees()),
        };
        let direction_y = match angle {
            x if x == 0.0 || x == PI => DirectionY::Parallel,
            x if x < PI => DirectionY::Up,
            x if x > PI => DirectionY::Down,
            _ => panic!("Unhandled y direction for angle {}°", angle.to_degrees()),
        };

        Self {
            grid_size,
            tile_size,
            tp,
            wc,
            angle,
            tan,
            direction_x,
            direction_y,
        }
    }

    fn initial_intersects(&self) -> InitialIntersects {
        let x_axis = self.initial_x_intersect();
        let y_axis = self.initial_y_intersect();
        InitialIntersects { x_axis, y_axis }
    }

    fn initial_x_intersect(&self) -> Option<(WorldCoords, TilePosition)> {
        if self.direction_x == DirectionX::Left && self.tp.x == 0
            || self.direction_x == DirectionX::Right && self.tp.x + 1 == self.grid_size.0
        {
            return None;
        }

        let dx = match self.direction_x {
            DirectionX::Left => Some(-self.tp.rel_x),
            DirectionX::Right => Some(self.tile_size - self.tp.rel_x),
            DirectionX::Parallel => None,
        };

        if let Some(dx) = dx {
            let dy = dx * self.tan;
            let wc = self.wc.translated(dx, dy);
            let tp = self.validated_tile_position(wc.to_signed_tile_position());
            if let Some(mut tp) = tp {
                self.normalize(&mut tp);
                Some((wc, tp))
            } else {
                None
            }
        } else {
            None
        }
    }

    fn initial_y_intersect(&self) -> Option<(WorldCoords, TilePosition)> {
        if self.direction_y == DirectionY::Down && self.tp.y == 0
            || self.direction_y == DirectionY::Up && self.tp.y + 1 == self.grid_size.1
        {
            return None;
        }

        let dy = match self.direction_y {
            DirectionY::Up => Some(self.tile_size - self.tp.rel_y),
            DirectionY::Down => Some(-self.tp.rel_y),
            DirectionY::Parallel => None,
        };

        if let Some(dy) = dy {
            let dx = dy / self.tan;
            let wc = self.wc.translated(dx, dy);
            let tp = self.validated_tile_position(wc.to_signed_tile_position());
            if let Some(mut tp) = tp {
                self.normalize(&mut tp);
                Some((wc, tp))
            } else {
                None
            }
        } else {
            None
        }
    }

    fn normalize(&self, tp: &mut TilePosition) {
        if self.direction_x == DirectionX::Left && tp.rel_x == 0.0 {
            tp.x -= 1;
            tp.rel_x += self.tile_size;
        }
        if self.direction_y == DirectionY::Down && tp.rel_y == 0.0 {
            tp.y -= 1;
            tp.rel_y += self.tile_size;
        }
    }

    fn validated_tile_position(&self, stp: SignedTilePosition) -> Option<TilePosition> {
        let tp: Option<TilePosition> = stp.into();
        if let Some(tp) = tp {
            self.bounded(tp)
        } else {
            None
        }
    }

    fn bounded(&self, tp: TilePosition) -> Option<TilePosition> {
        if tp.x < self.grid_size.0 && tp.y < self.grid_size.1 {
            Some(tp)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init_centered_3x3(angle_deg: f32) -> Intersections {
        let grid_size = (3, 3);
        let tiles_size = 1.0;
        let tp = TilePosition::new(1, 1, 0.5, 0.5);

        Intersections::new(grid_size, tiles_size, tp, angle_deg.to_radians())
    }

    #[test]
    fn angle_directions() {
        for (angle, dir_x, dir_y) in vec![
            (-80.0, DirectionX::Right, DirectionY::Down),
            (0.0, DirectionX::Right, DirectionY::Parallel),
            (30.0, DirectionX::Right, DirectionY::Up),
            (45.0, DirectionX::Right, DirectionY::Up),
            (80.0, DirectionX::Right, DirectionY::Up),
            (90.0, DirectionX::Parallel, DirectionY::Up),
            (120.0, DirectionX::Left, DirectionY::Up),
            (180.0, DirectionX::Left, DirectionY::Parallel),
            (210.0, DirectionX::Left, DirectionY::Down),
            (270.0, DirectionX::Parallel, DirectionY::Down),
            (290.0, DirectionX::Right, DirectionY::Down),
            (310.0, DirectionX::Right, DirectionY::Down),
            (360.0, DirectionX::Right, DirectionY::Parallel),
            (390.0, DirectionX::Right, DirectionY::Up),
            (405.0, DirectionX::Right, DirectionY::Up),
        ] {
            let Intersections {
                direction_x,
                direction_y,
                ..
            } = init_centered_3x3(angle);
            assert_eq!((dir_x, dir_y), (direction_x, direction_y));
        }
    }

    #[test]
    fn starting_intersections() {
        let verify: Vec<(f32, Option<TilePosition>, Option<TilePosition>)> = vec![
            (0.0, Some(((2, 0.000), (1, 0.500)).into()), None),
            (
                30.0,
                Some(((2, 0.000), (1, 0.789)).into()),
                Some(((2, 0.366), (2, 0.000)).into()),
            ),
            (
                45.0,
                Some(((2, 0.000), (2, 0.000)).into()),
                Some(((2, 0.000), (2, 0.000)).into()),
            ),
            (
                60.0,
                Some(((2, 0.000), (2, 0.366)).into()),
                Some(((1, 0.789), (2, 0.000)).into()),
            ),
            (90.0, None, Some(((1, 0.500), (2, 0.000)).into())),
            (
                120.0,
                Some(((0, 1.000), (2, 0.366)).into()),
                Some(((1, 0.211), (2, 0.000)).into()),
            ),
            (
                135.0,
                Some(((0, 1.000), (2, 0.000)).into()),
                Some(((0, 1.000), (2, 0.000)).into()),
            ),
            (
                150.0,
                Some(((0, 1.0), (1, 0.789)).into()),
                Some(((0, 0.634), (2, 0.000)).into()),
            ),
            (180.0, Some(((0, 1.000), (1, 0.500)).into()), None),
            (
                210.0,
                Some(((0, 1.000), (1, 0.211)).into()),
                Some(((0, 0.634), (0, 1.000)).into()),
            ),
            (
                225.0,
                Some(((0, 1.000), (0, 1.000)).into()),
                Some(((0, 1.000), (0, 1.000)).into()),
            ),
            (270.0, None, Some(((1, 0.500), (0, 1.000)).into())),
            (
                330.0,
                Some(((2, 0.000), (1, 0.211)).into()),
                Some(((2, 0.366), (0, 1.000)).into()),
            ),
        ];
        for (angle, x, y) in verify {
            let intersections = init_centered_3x3(angle);
            let InitialIntersects { x_axis, y_axis } = intersections.initial_intersects();
            let x_axis = x_axis.map(|(_, tp)| tp);
            let y_axis = y_axis.map(|(_, tp)| tp);
            assert_eq!(x_axis, x);
            assert_eq!(y_axis, y);
        }
    }
}
