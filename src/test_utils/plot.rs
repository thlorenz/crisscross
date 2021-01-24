use crate::{
    canvas::{BLUE, DARK_GOLDENROD, GRAY, LIGHT_GRAY},
    ray::Ray,
    rays::rays_from,
    util::round,
    AngleRad, BeamIntersect, Grid, TilePosition,
};

use super::canvas::Canvas;

const SCALE: f32 = 180.0;

pub enum PlotType {
    File,
    Log,
    Memory,
}

pub fn tile_position_label(tp: &TilePosition) -> String {
    let TilePosition { x, y, rel_x, rel_y } = tp;
    let tp = TilePosition {
        x: *x,
        y: *y,
        rel_x: round(*rel_x, 1),
        rel_y: round(*rel_y, 1),
    };

    format!("at_{}+{}_{}+{}", tp.x, tp.rel_x, tp.y, tp.rel_y)
}

pub fn plot_ray(
    label: &str,
    grid: &Grid,
    origin: &TilePosition,
    angle: f32,
    mut tps: Vec<&TilePosition>,
    plot_type: PlotType,
) -> Canvas {
    tps.sort_by(|tp1, tp2| {
        let dist1 = origin.distance(*tp1, grid.tile_size);
        let dist2 = origin.distance(*tp2, grid.tile_size);
        dist1.partial_cmp(&dist2).unwrap()
    });

    let mut canvas = Canvas::new(grid.clone(), SCALE);
    canvas.plot_ray(&origin, &tps);

    let angle = angle.to_degrees().round() as i32;
    let tp = tile_position_label(origin);
    let img_name = format!(
        "{}_{}x{}_{}_{}_deg.png",
        label, grid.cols, grid.rows, tp, angle,
    );
    match plot_type {
        PlotType::File => canvas.save(&img_name).expect("save ray"),
        PlotType::Log => canvas.log(&img_name).expect("log ray"),
        PlotType::Memory => {}
    };

    canvas
}

pub fn plot_rays_origins(
    grid: &Grid,
    center: &TilePosition,
    width: f32,
    angle: &AngleRad,
    rays: &mut Vec<Ray>,
    plot_type: PlotType,
) -> Canvas {
    let mut canvas = Canvas::new(grid.clone(), SCALE);

    let angle = angle.degrees().round() as i32;
    let tp = tile_position_label(center);
    let img_name = format!(
        "ray_origins_{}x{}_{}_{:2}w_{}_deg.png",
        grid.cols, grid.rows, tp, width, angle
    );
    canvas.plot_origin(center);

    for ray in rays {
        canvas.plot_tile_position(&ray.tp, BLUE);
        if let Some(next) = ray.next_intersect() {
            canvas.plot_tile_position(&next, GRAY);
            canvas.plot_line(&ray.tp, &next, LIGHT_GRAY);
        }
    }

    match plot_type {
        PlotType::File => canvas.save(&img_name).expect("save ray"),
        PlotType::Log => canvas.log(&img_name).expect("log ray"),
        PlotType::Memory => {}
    };

    canvas
}

pub fn plot_beam(
    grid: &Grid,
    center: &TilePosition,
    width: f32,
    angle: &AngleRad,
    beam_intersects: &Vec<BeamIntersect>,
    plot_type: PlotType,
) {
    let mut rays: Vec<Ray> = rays_from(center, &grid, width, &angle);
    let mut canvas = plot_rays_origins(grid, center, width, angle, &mut rays, PlotType::Memory);

    for BeamIntersect(ray_idx, tp) in beam_intersects {
        let ray = rays.get(*ray_idx).unwrap();
        canvas.plot_tile_position_bold(&tp, DARK_GOLDENROD);
        canvas.plot_line(&ray.tp, &tp, LIGHT_GRAY);
    }

    let tp = tile_position_label(center);
    let img_name = format!(
        "beam_{}x{}_{}_{}_deg.png",
        grid.cols, grid.rows, tp, angle.0,
    );

    match plot_type {
        PlotType::File => canvas.save(&img_name).expect("save beam"),
        PlotType::Log => canvas.log(&img_name).expect("log beam"),
        PlotType::Memory => {}
    };
}
