use std::f32::consts::PI;

use crate::{
    angle::{DirectionX, DirectionY},
    position::WorldCoords,
    ray::Ray,
    util::round,
    AngleRad, Grid, TilePosition,
};

const RAY_PRECISION: usize = 8;

pub fn rays_from(center: &TilePosition, grid: &Grid, width: f32, angle: &AngleRad) -> Vec<Ray> {
    debug_assert!(width > 0.0, "width needs to be > 0");

    let center_wc = WorldCoords::from_tile_position(center, grid.tile_size);

    let left_rad: AngleRad = angle.perpendicular();
    let (left_sin, left_cos) = (
        round(left_rad.sin(), RAY_PRECISION),
        round(left_rad.cos(), RAY_PRECISION),
    );

    // sections on each side
    let sections = (width.ceil() / grid.tile_size.floor()).max(1.0).ceil();
    let section_width = (width / 2.0) / sections;
    #[allow(
        clippy::as_conversions,
        clippy::cast_sign_loss,
        clippy::clippy::cast_possible_truncation
    )]
    let sections = sections as i16;

    let (fx, fy) = {
        (
            match DirectionX::from(&left_rad) {
                DirectionX::Left | DirectionX::Parallel => 1.0,
                DirectionX::Right => -1.0,
            },
            match DirectionY::from(&left_rad) {
                DirectionY::Up | DirectionY::Parallel => {
                    // Honestly I've got no idea why this is necessary :(
                    if angle.0 > 1.5 * PI {
                        -1.0
                    } else {
                        1.0
                    }
                }
                DirectionY::Down => -1.0,
            },
        )
    };
    #[allow(clippy::integer_arithmetic)]
    (-sections..=sections)
        .filter_map(|idx| {
            let len = section_width * f32::from(idx);
            let dx = left_sin * len * fx;
            let dy = left_cos * len * fy;
            let tp = center_wc
                .translated(dx, dy)
                .bounds_checked(grid)?
                .to_tile_position()
                .ok()?;
            Some(Ray::new(grid.clone(), tp, angle.clone()))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::util::round_tp;

    use super::*;

    fn rays_for_angle(
        center: &TilePosition,
        grid: &Grid,
        width: f32,
        angle: f32,
    ) -> Vec<TilePosition> {
        let rays = rays_from(center, grid, width, &angle.into());
        #[cfg(feature = "plot")]
        {
            use crate::plot::{plot_rays_origins, PlotType};
            let mut rays = rays_from(center, grid, width, &angle.into());
            plot_rays_origins(
                &grid,
                center,
                width,
                &angle.into(),
                &mut rays,
                PlotType::File,
            );
        }
        rays.iter().map(|ray| round_tp(&ray.tp)).collect()
    }

    #[test]
    fn rays_from_width_smaller_than_tile_isolate() {
        let center = TilePosition::new(1, 1, 0.5, 0.5);
        let grid = Grid::new(4, 4, 1.0);
        let width = grid.tile_size * 0.8;

        // Right/Down at 315
        let angle = 315_f32.to_radians();
        assert_eq!(
            rays_for_angle(&center, &grid, width, angle),
            [
                ((1, 0.217), (1, 0.217)).into(),
                ((1, 0.500), (1, 0.500)).into(),
                ((1, 0.783), (1, 0.783)).into()
            ]
        );
    }
    #[test]
    fn rays_from_width_smaller_than_tile() {
        let center = TilePosition::new(1, 1, 0.5, 0.5);
        let grid = Grid::new(4, 4, 1.0);
        let width = grid.tile_size * 0.8;

        // To the right
        let angle = 0.0;
        assert_eq!(
            rays_for_angle(&center, &grid, width, angle),
            [
                ((1, 0.500), (1, 0.900)).into(),
                ((1, 0.500), (1, 0.500)).into(),
                ((1, 0.500), (1, 0.100)).into(),
            ],
        );
        //
        // To the left
        let angle = 180_f32.to_radians();
        assert_eq!(
            rays_for_angle(&center, &grid, width, angle),
            [
                ((1, 0.500), (1, 0.100)).into(),
                ((1, 0.500), (1, 0.500)).into(),
                ((1, 0.500), (1, 0.900)).into()
            ],
        );

        // Up
        let angle = 90_f32.to_radians();
        assert_eq!(
            rays_for_angle(&center, &grid, width, angle),
            [
                ((1, 0.100), (1, 0.500)).into(),
                ((1, 0.500), (1, 0.500)).into(),
                ((1, 0.900), (1, 0.500)).into(),
            ],
        );

        // Down
        let angle = 270_f32.to_radians();
        assert_eq!(
            rays_for_angle(&center, &grid, width, angle),
            [
                ((1, 0.900), (1, 0.500)).into(),
                ((1, 0.500), (1, 0.500)).into(),
                ((1, 0.100), (1, 0.500)).into()
            ],
        );

        // Right/Up at 45
        let angle = 45_f32.to_radians();
        assert_eq!(
            rays_for_angle(&center, &grid, width, angle),
            [
                ((1, 0.217), (1, 0.783)).into(),
                ((1, 0.500), (1, 0.500)).into(),
                ((1, 0.783), (1, 0.217)).into()
            ]
        );

        // Left/Up at 120
        let angle = 120_f32.to_radians();
        assert_eq!(
            rays_for_angle(&center, &grid, width, angle),
            [
                ((1, 0.154), (1, 0.300)).into(),
                ((1, 0.500), (1, 0.500)).into(),
                ((1, 0.846), (1, 0.700)).into()
            ]
        );

        // Left/Down at 225
        let angle = 225_f32.to_radians();
        assert_eq!(
            rays_for_angle(&center, &grid, width, angle),
            [
                ((1, 0.783), (1, 0.217)).into(),
                ((1, 0.500), (1, 0.500)).into(),
                ((1, 0.217), (1, 0.783)).into()
            ]
        );

        // Right/Down at 315
        let angle = 315_f32.to_radians();
        assert_eq!(
            rays_for_angle(&center, &grid, width, angle),
            [
                ((1, 0.217), (1, 0.217)).into(),
                ((1, 0.500), (1, 0.500)).into(),
                ((1, 0.783), (1, 0.783)).into()
            ]
        );

        //
        // Checking edge cases regarding the oddity of angle > 1.5 * PI
        //

        // Left/Down
        let angle = 269_f32.to_radians();
        assert_eq!(
            rays_for_angle(&center, &grid, width, angle),
            [
                ((1, 0.900), (1, 0.493)).into(),
                ((1, 0.500), (1, 0.500)).into(),
                ((1, 0.100), (1, 0.507)).into()
            ]
        );

        let angle = 271_f32.to_radians();
        assert_eq!(
            rays_for_angle(&center, &grid, width, angle),
            [
                ((1, 0.100), (1, 0.493)).into(),
                ((1, 0.500), (1, 0.500)).into(),
                ((1, 0.900), (1, 0.507)).into()
            ]
        );

        let angle = 359_f32.to_radians();
        assert_eq!(
            rays_for_angle(&center, &grid, width, angle),
            [
                ((1, 0.493), (1, 0.100)).into(),
                ((1, 0.500), (1, 0.500)).into(),
                ((1, 0.507), (1, 0.900)).into()
            ]
        );

        let angle = 361_f32.to_radians();
        assert_eq!(
            rays_for_angle(&center, &grid, width, angle),
            [
                ((1, 0.507), (1, 0.100)).into(),
                ((1, 0.500), (1, 0.500)).into(),
                ((1, 0.493), (1, 0.900)).into()
            ]
        );
    }

    #[test]
    fn rays_from_width_same_as_tile() {
        let center = TilePosition::new(1, 1, 0.5, 0.5);
        let grid = Grid::new(4, 4, 1.0);
        let width = grid.tile_size;

        let angle = 0.0;
        assert_eq!(
            rays_for_angle(&center, &grid, width, angle),
            [
                ((1, 0.500), (2, 0.000)).into(),
                ((1, 0.500), (1, 0.500)).into(),
                ((1, 0.500), (1, 0.000)).into()
            ]
        );

        let angle = 120_f32.to_radians();
        assert_eq!(
            rays_for_angle(&center, &grid, width, angle),
            [
                ((1, 0.067), (1, 0.250)).into(),
                ((1, 0.500), (1, 0.500)).into(),
                ((1, 0.933), (1, 0.750)).into()
            ]
        );
    }

    #[test]
    fn rays_from_width_larger_than_tile() {
        let center = TilePosition::new(1, 1, 0.5, 0.5);
        let grid = Grid::new(4, 4, 1.0);
        let width = grid.tile_size * 2.0;

        let angle = 0.0;
        assert_eq!(
            rays_for_angle(&center, &grid, width, angle),
            [
                ((1, 0.500), (2, 0.500)).into(),
                ((1, 0.500), (2, 0.000)).into(),
                ((1, 0.500), (1, 0.500)).into(),
                ((1, 0.500), (1, 0.000)).into(),
                ((1, 0.500), (0, 0.500)).into()
            ]
        );

        let angle = 120_f32.to_radians();
        assert_eq!(
            rays_for_angle(&center, &grid, width, angle),
            [
                ((0, 0.634), (1, 0.000)).into(),
                ((1, 0.067), (1, 0.250)).into(),
                ((1, 0.500), (1, 0.500)).into(),
                ((1, 0.933), (1, 0.750)).into(),
                ((2, 0.366), (2, 0.000)).into()
            ]
        );
    }

    #[test]
    fn rays_bounds() {
        let grid = Grid::new(4, 4, 1.0);

        let angle = 0.0;
        let center = TilePosition::new(0, 0, 0.0, 0.0);
        let width = grid.tile_size * 0.8;
        assert_eq!(
            rays_for_angle(&center, &grid, width, angle),
            [
                ((0, 0.000), (0, 0.400)).into(),
                ((0, 0.000), (0, 0.000)).into(),
            ]
        );

        let center = TilePosition::new(1, 1, 0.5, 0.5);
        let width = grid.tile_size * 10.0;
        assert_eq!(
            rays_for_angle(&center, &grid, width, angle),
            [
                ((1, 0.500), (3, 0.500)).into(),
                ((1, 0.500), (3, 0.000)).into(),
                ((1, 0.500), (2, 0.500)).into(),
                ((1, 0.500), (2, 0.000)).into(),
                ((1, 0.500), (1, 0.500)).into(),
                ((1, 0.500), (1, 0.000)).into(),
                ((1, 0.500), (0, 0.500)).into(),
                ((1, 0.500), (0, 0.000)).into()
            ]
        );

        let angle = 90_f32.to_radians();
        let center = TilePosition::new(2, 2, 0.5, 0.5);
        let width = grid.tile_size * 10.0;
        assert_eq!(
            rays_for_angle(&center, &grid, width, angle),
            [
                ((0, 0.000), (2, 0.500)).into(),
                ((0, 0.500), (2, 0.500)).into(),
                ((1, 0.000), (2, 0.500)).into(),
                ((1, 0.500), (2, 0.500)).into(),
                ((2, 0.000), (2, 0.500)).into(),
                ((2, 0.500), (2, 0.500)).into(),
                ((3, 0.000), (2, 0.500)).into(),
                ((3, 0.500), (2, 0.500)).into()
            ]
        );

        let angle = 315_f32.to_radians();
        let center = TilePosition::new(0, 2, 0.5, 0.5);
        let width = grid.tile_size * 10.0;
        assert_eq!(
            rays_for_angle(&center, &grid, width, angle),
            [
                ((0, 0.146), (2, 0.146)).into(),
                ((0, 0.500), (2, 0.500)).into(),
                ((0, 0.854), (2, 0.854)).into(),
                ((1, 0.207), (3, 0.207)).into(),
                ((1, 0.561), (3, 0.561)).into(),
                ((1, 0.914), (3, 0.914)).into()
            ]
        );
    }
}
