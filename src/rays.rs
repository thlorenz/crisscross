use crate::{
    angle::{DirectionX, DirectionY},
    position::WorldCoords,
    ray::Ray,
    util::round,
    AngleRad, Grid, TilePosition,
};

const RAY_PRECISION: usize = 8;

pub fn rays_from(center: &TilePosition, grid: &Grid, width: f32, angle: AngleRad) -> Vec<Ray> {
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
    let sections = sections as i16;

    #[allow(
        clippy::as_conversions,
        clippy::cast_sign_loss,
        clippy::clippy::cast_possible_truncation
    )]
    let (fx, fy) = {
        (
            match DirectionX::from(&left_rad) {
                DirectionX::Left | DirectionX::Parallel => 1.0,
                DirectionX::Right => -1.0,
            },
            match DirectionY::from(&left_rad) {
                DirectionY::Up | DirectionY::Parallel => 1.0,
                DirectionY::Down => -1.0,
            },
        )
    };

    (-sections..=sections)
        .filter_map(|idx| {
            let len = section_width * f32::from(idx);
            let dx = left_sin * len * fx;
            let dy = left_cos * len * fy;
            let tp = center_wc.translated(dx, dy).to_tile_position().ok()?;
            Some(Ray::new(grid.clone(), tp, angle.clone()))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::util::round_tp;

    use super::*;

    fn rays_for_angle<T: Into<AngleRad>>(
        center: &TilePosition,
        grid: &Grid,
        width: f32,
        angle: T,
    ) -> Vec<TilePosition> {
        let rays = rays_from(center, grid, width, angle.into());
        rays.iter().map(|ray| round_tp(&ray.tp)).collect()
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
                ((1, 0.217), (1, 0.783)).into(),
                ((1, 0.500), (1, 0.500)).into(),
                ((1, 0.783), (1, 0.217)).into()
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
}
