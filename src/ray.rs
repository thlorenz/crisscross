use std::convert::TryInto;

use crate::{
    angle::{DirectionX, DirectionY},
    grid::Grid,
    position::{SignedTilePosition, TilePosition, WorldCoords},
    util::floats_equal,
    AngleRad,
};

fn normalize_zeros(tp: &mut SignedTilePosition) {
    // Avoid (-0.0)
    if floats_equal(tp.rel_x, 0.0) {
        tp.rel_x = 0.0;
    }
    if floats_equal(tp.rel_y, 0.0) {
        tp.rel_y = 0.0;
    }
}

/// Assumes origin (0, 0) is at bottom left.
/// Assumes relative tile position are based on (0.0, 0.0) being located at the bottom left of each
/// tile.
#[derive(Debug)]
pub struct Ray {
    tan: f32,
    direction_x: DirectionX,
    direction_y: DirectionY,
    grid: Grid,
    intersect_x: Option<TilePosition>,
    intersect_y: Option<TilePosition>,
    delta_x_axis_intersect: Option<SignedTilePosition>,
    delta_y_axis_intersect: Option<SignedTilePosition>,
    pub(crate) wc: WorldCoords,
    pub(crate) tp: TilePosition,
}

//
// Constructor API
//
impl Ray {
    pub(crate) fn new<T>(grid: Grid, tp: TilePosition, angle: T) -> Self
    where
        T: Into<AngleRad>,
    {
        let wc = WorldCoords::from_tile_position(&tp, grid.tile_size);
        let angle = (angle).into().clamp();
        let tan = angle.0.tan();

        let direction_x: DirectionX = (&angle).into();
        let direction_y: DirectionY = (&angle).into();

        let delta_x_axis_intersects = {
            (match direction_x {
                DirectionX::Left => Some(-grid.tile_size),
                DirectionX::Right => Some(grid.tile_size),
                DirectionX::Parallel => None,
            })
            .map(|dx| {
                let dy = tan * dx;
                let mut stp = WorldCoords::new(dx, dy, grid.tile_size).to_signed_tile_position();
                normalize_zeros(&mut stp);
                stp
            })
        };

        let delta_y_axis_intersects = {
            (match direction_y {
                DirectionY::Up => Some(grid.tile_size),
                DirectionY::Down => Some(-grid.tile_size),
                DirectionY::Parallel => None,
            })
            .map(|dy| {
                let dx = dy / tan;
                let mut stp = WorldCoords::new(dx, dy, grid.tile_size).to_signed_tile_position();
                normalize_zeros(&mut stp);
                stp
            })
        };

        let mut me = Self {
            grid,
            tp,
            wc,
            tan,
            direction_x,
            direction_y,
            intersect_x: None,
            intersect_y: None,
            delta_x_axis_intersect: delta_x_axis_intersects,
            delta_y_axis_intersect: delta_y_axis_intersects,
        };
        me.intersect_x = me.initial_x_intersect();
        me.intersect_y = me.initial_y_intersect();

        me
    }
}

//
// Inital Intersects
//
impl Ray {
    #[allow(clippy::integer_arithmetic)]
    fn initial_x_intersect(&self) -> Option<TilePosition> {
        if self.direction_x == DirectionX::Left && self.tp.x == 0
            || self.direction_x == DirectionX::Right && self.tp.x + 1 == self.grid.cols
        {
            return None;
        }

        let dx = match self.direction_x {
            DirectionX::Left => Some(-self.tp.rel_x),
            DirectionX::Right => Some(self.grid.tile_size - self.tp.rel_x),
            DirectionX::Parallel => None,
        }?;

        let dy = dx * self.tan;
        let wc = self.wc.translated(dx, dy);
        self.normalized_valid_tile_position(&wc)
    }

    #[allow(clippy::integer_arithmetic)]
    fn initial_y_intersect(&self) -> Option<TilePosition> {
        if self.direction_y == DirectionY::Down && self.tp.y == 0
            || self.direction_y == DirectionY::Up && self.tp.y + 1 == self.grid.rows
        {
            return None;
        }

        let dy = match self.direction_y {
            DirectionY::Up => Some(self.grid.tile_size - self.tp.rel_y),
            DirectionY::Down => Some(-self.tp.rel_y),
            DirectionY::Parallel => None,
        }?;

        let dx = dy / self.tan;
        let wc = self.wc.translated(dx, dy);
        self.normalized_valid_tile_position(&wc)
    }
}

//
// Validators/Normalizers
//
impl Ray {
    fn normalized_valid_tile_position(&self, wc: &WorldCoords) -> Option<TilePosition> {
        let mut stp = wc.to_signed_tile_position();
        self.normalize(&mut stp);
        self.validated_tile_position(stp)
    }

    fn validated_tile_position(&self, stp: SignedTilePosition) -> Option<TilePosition> {
        match stp.try_into() {
            Ok(tp) => self.bounded(tp),
            Err(_) => None,
        }
    }

    const fn bounded(&self, tp: TilePosition) -> Option<TilePosition> {
        if tp.x < self.grid.cols && tp.y < self.grid.rows {
            Some(tp)
        } else {
            None
        }
    }

    #[allow(clippy::integer_arithmetic)]
    fn normalize(&self, tp: &mut SignedTilePosition) {
        if self.direction_x == DirectionX::Left && floats_equal(tp.rel_x, 0.0) {
            tp.x -= 1;
            tp.rel_x += self.grid.tile_size;
        }
        if self.direction_y == DirectionY::Down && floats_equal(tp.rel_y, 0.0) {
            tp.y -= 1;
            tp.rel_y += self.grid.tile_size;
        }
        normalize_zeros(tp);
    }
}

//
// Iteration
//
enum Axis {
    X,
    Y,
}

impl Ray {
    pub(crate) fn next_intersect(&mut self) -> Option<TilePosition> {
        let closest_axis = match (&self.intersect_x, &self.intersect_y) {
            (None, None) => None,
            (None, Some(_)) => Some(Axis::Y),
            (Some(_), None) => Some(Axis::X),
            (Some(ref tpx), Some(ref tpy)) => {
                let dx = self.tp.distance_global(tpx, self.grid.tile_size);
                let dy = self.tp.distance_global(tpy, self.grid.tile_size);
                if dx < dy {
                    Some(Axis::X)
                } else {
                    Some(Axis::Y)
                }
            }
        };
        match closest_axis {
            Some(Axis::X) => {
                let next = self.intersect_x.clone();
                self.update_intersect_x();
                next
            }
            Some(Axis::Y) => {
                let next = self.intersect_y.clone();
                self.update_intersect_y();
                next
            }
            None => None,
        }
    }

    fn update_intersect_x(&mut self) {
        self.intersect_x = self.next_intersect_for(&self.intersect_x, &self.delta_x_axis_intersect);
    }

    fn update_intersect_y(&mut self) {
        self.intersect_y = self.next_intersect_for(&self.intersect_y, &self.delta_y_axis_intersect);
    }

    fn next_intersect_for(
        &self,
        intersect: &Option<TilePosition>,
        delta: &Option<SignedTilePosition>,
    ) -> Option<TilePosition> {
        match (intersect, delta) {
            (None, _) | (_, None) => None,
            (Some(ref intersect), Some(ref delta)) => {
                let stp = intersect + delta;
                // convert back and forth to world coords to ensure that rel_x,rel_y <= tile_size
                let wc = WorldCoords::from_signed_tile_position(&stp, self.grid.tile_size);
                self.normalized_valid_tile_position(&wc)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "plot")]
    use crate::plot::{plot_ray, PlotType};

    use crate::util::{round_ostp, round_otp};

    use super::*;

    fn init_centered_3x3(angle_deg: f32) -> Ray {
        let tile_size = 1.0;
        let grid = Grid::new(3, 3, tile_size);
        let tp = TilePosition::new(1, 1, 0.5, 0.5);

        Ray::new(grid, tp, angle_deg.to_radians())
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
            let Ray {
                direction_x,
                direction_y,
                ..
            } = init_centered_3x3(angle);
            assert_eq!((dir_x, dir_y), (direction_x, direction_y));
        }
    }
    #[test]
    fn starting_intersections() {
        let test_cases: Vec<(f32, Option<TilePosition>, Option<TilePosition>)> = vec![
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
        for (angle, x, y) in test_cases {
            let ray = init_centered_3x3(angle);
            let Ray {
                intersect_x,
                intersect_y,
                ..
            } = ray;

            #[cfg(feature = "plot")]
            {
                let tps: Vec<&TilePosition> = vec![&intersect_x, &intersect_y]
                    .into_iter()
                    .flat_map(|x| x)
                    .collect();

                let Ray { grid, tp, .. } = ray;

                plot_ray(
                    "starting_intersections",
                    &grid,
                    &tp,
                    angle.to_radians(),
                    tps,
                    PlotType::File,
                );
            }

            assert_eq!(round_otp(intersect_x), x);
            assert_eq!(round_otp(intersect_y), y);
        }
    }

    #[test]
    fn intersection_deltas() {
        let test_cases: Vec<(f32, Option<SignedTilePosition>, Option<SignedTilePosition>)> = vec![
            (0.0, Some(((1, 0.00), (0, 0.00)).into()), None),
            (
                30.0,
                Some(((1, 0.00), (0, 0.577)).into()),
                Some(((1, 0.732), (1, 0.00)).into()),
            ),
            (
                45.0,
                Some(((1, 0.00), (1, 0.000)).into()),
                Some(((1, 0.00), (1, 0.000)).into()),
            ),
            (
                60.0,
                Some(((1, 0.00), (1, 0.732)).into()),
                Some(((0, 0.577), (1, 0.00)).into()),
            ),
            (90.0, None, Some(((0, 0.00), (1, 0.00)).into())),
            (
                135.0,
                Some(((-1, 0.00), (1, 0.000)).into()),
                Some(((-1, 0.00), (1, 0.000)).into()),
            ),
            (
                150.0,
                Some(((-1, 0.00), (0, 0.577)).into()),
                Some(((-1, -0.732), (1, 0.000)).into()),
            ),
            (180.0, Some(((-1, 0.00), (0, 0.00)).into()), None),
            (
                225.0,
                Some(((-1, 0.000), (0, -1.000)).into()),
                Some(((-1, 0.000), (-1, 0.000)).into()),
            ),
            (
                210.0,
                Some(((-1, 0.00), (0, -0.577)).into()),
                Some(((-1, -0.732), (-1, 0.000)).into()),
            ),
            (270.0, None, Some(((0, 0.00), (-1, 0.00)).into())),
            (
                315.0,
                Some(((1, 0.000), (-1, 0.000)).into()),
                Some(((0, 1.00), (-1, 0.000)).into()),
            ),
            (
                330.0,
                Some(((1, 0.00), (0, -0.577)).into()),
                Some(((1, 0.732), (-1, 0.000)).into()),
            ),
        ];
        for (angle, dx, dy) in test_cases {
            let Ray {
                delta_x_axis_intersect: delta_x_axis_intersects,
                delta_y_axis_intersect: delta_y_axis_intersects,
                ..
            } = init_centered_3x3(angle);
            assert_eq!(round_ostp(delta_x_axis_intersects), dx);
            assert_eq!(round_ostp(delta_y_axis_intersects), dy);
        }
    }
}
