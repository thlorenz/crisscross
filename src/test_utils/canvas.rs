use std::env;

use image::{ImageError, ImageFormat, Rgb, RgbImage};
use imageproc::{
    drawing::{
        draw_filled_circle_mut, draw_filled_rect_mut, draw_hollow_circle_mut, draw_line_segment_mut,
    },
    rect::Rect,
};

use crate::{Grid, TilePosition};

type ImageResult<T> = std::result::Result<T, ImageError>;

fn point_from_tile_position(tp: &TilePosition) -> (f32, f32) {
    (tp.x as f32 + tp.rel_x, tp.y as f32 + tp.rel_y)
}

type Color = Rgb<u8>;
pub const WHITE: Color = Rgb([0xff, 0xff, 0xff]);
pub const WHITE_SMOKE: Color = Rgb([0xf5, 0xf5, 0xf5]);
pub const PAPAYA_WHIIP: Color = Rgb([0xff, 0xef, 0xd5]);

pub const LIGHT_GRAY: Color = Rgb([0xaa, 0xaa, 0xaa]);
pub const GRAY: Color = Rgb([0x80, 0x80, 0x80]);
pub const DARK_GRAY: Color = Rgb([0x20, 0x20, 0x20]);
pub const BLACK: Color = Rgb([0x00, 0x00, 0x00]);

pub const BLUE: Color = Rgb([0x00, 0x00, 0xff]);
pub const GREEN: Color = Rgb([0x00, 0xff, 0x00]);

pub struct Canvas {
    grid: Grid,
    scale: f32,
    image: RgbImage,
}

impl Canvas {
    pub fn new(grid: Grid, scale: f32) -> Self {
        let image = RgbImage::new(
            (grid.width * scale as f64) as u32,
            (grid.height * scale as f64) as u32,
        );
        Self { grid, image, scale }
    }

    fn clear(&mut self, color: Color) {
        let w = self.image.width();
        let h = self.image.height();
        draw_filled_rect_mut(&mut self.image, Rect::at(0, 0).of_size(w, h), color);
    }

    fn draw_grid(&mut self, color: Color) {
        for row in 0..self.grid.rows {
            let y = row as f32 * self.grid.tile_size;
            let start = (0.0, y);
            let end = (self.grid.width as f32, y);
            self.draw_line(start, end, color);
        }
        for col in 0..self.grid.cols {
            let x = col as f32 * self.grid.tile_size;
            let start = (x, 0.0);
            let end = (x, self.grid.height as f32);
            self.draw_line(start, end, color);
        }
    }

    fn draw_line(&mut self, start: (f32, f32), end: (f32, f32), color: Color) {
        let start = self.normalize(start);
        let end = self.normalize(end);
        draw_line_segment_mut(&mut self.image, start, end, color);
    }

    fn normalize_point(&self, point: (f32, f32), radius: f32) -> ((i32, i32), i32) {
        let radius = (radius * self.scale).round() as i32;
        let (x, y) = self.normalize_i32(point);
        let point = (x, y);
        (point, radius * 2)
    }

    fn draw_point(&mut self, point: (f32, f32), radius: f32, color: Color) {
        let (point, diameter) = self.normalize_point(point, radius);
        // draw_filled_circle_mut arg is called radius but treated like diameter
        draw_filled_circle_mut(&mut self.image, point, diameter as i32, color);
    }

    fn draw_point_border(&mut self, point: (f32, f32), radius: f32, color: Color) {
        let (point, diameter) = self.normalize_point(point, radius);
        draw_hollow_circle_mut(&mut self.image, point, diameter as i32, color);
    }

    fn draw_tile_position(&mut self, tp: &TilePosition, radius: f32, color: Color) {
        let point = point_from_tile_position(tp);
        self.draw_point(point, radius, color);
    }

    fn draw_tile_position_border(&mut self, tp: &TilePosition, radius: f32, color: Color) {
        let point = point_from_tile_position(tp);
        self.draw_point_border(point, radius, color);
    }

    fn draw_tile_positions(&mut self, tps: &Vec<&TilePosition>, radius: f32, color: Color) {
        for tp in tps {
            self.draw_tile_position(tp, radius, color)
        }
    }

    fn draw_tile_position_opt(&mut self, tp: &Option<TilePosition>, radius: f32, color: Color) {
        if let Some(tp) = tp {
            let point = point_from_tile_position(tp);
            self.draw_point(point, radius, color);
        }
    }

    fn draw_tile_position_opts(&mut self, tps: &[Option<TilePosition>], radius: f32, color: Color) {
        for tp in tps {
            self.draw_tile_position_opt(tp, radius, color)
        }
    }

    fn normalize(&self, (x, y): (f32, f32)) -> (f32, f32) {
        let x = x * self.scale;
        let y = (self.grid.height - y as f64) as f32 * self.scale;
        (x, y)
    }

    fn normalize_i32(&self, (x, y): (f32, f32)) -> (i32, i32) {
        let x = x * self.scale;
        let y = (self.grid.height - y as f64) as f32 * self.scale;
        (x as i32, y as i32)
    }

    // API
    pub fn plot_origin(&mut self, origin: &TilePosition) {
        let radius = 0.04;
        self.clear(WHITE_SMOKE);
        self.draw_grid(LIGHT_GRAY);
        self.draw_tile_position(&origin, radius, GREEN);
        self.draw_tile_position_border(&origin, radius, BLUE);
    }

    pub fn plot_origin_and_intersections(
        &mut self,
        origin: &TilePosition,
        tps: &Vec<&TilePosition>,
    ) {
        self.plot_origin(origin);
        self.draw_tile_positions(tps, 0.005, DARK_GRAY);
    }

    pub fn plot_origin_and_intersection_opts(
        &mut self,
        origin: &TilePosition,
        tps: &[Option<TilePosition>],
    ) {
        self.plot_origin(origin);
        self.draw_tile_position_opts(tps, 0.002, DARK_GRAY);
    }

    pub fn plot_ray(&mut self, origin: &TilePosition, tps: &Vec<&TilePosition>) {
        self.plot_origin(origin);

        let start = point_from_tile_position(origin);
        let end = point_from_tile_position(tps.last().unwrap());
        self.draw_line(start, end, LIGHT_GRAY);

        self.draw_tile_positions(tps, 0.005, DARK_GRAY);
    }

    pub fn plot_tile_position(&mut self, tp: &TilePosition, color: Color) {
        self.draw_tile_position(tp, 0.005, color)
    }

    pub fn plot_tile_positions(&mut self, tps: &Vec<&TilePosition>) {
        self.draw_tile_positions(tps, 0.005, BLUE)
    }

    pub fn plot_line(&mut self, start: &TilePosition, end: &TilePosition, color: Color) {
        let start = point_from_tile_position(start);
        let end = point_from_tile_position(end);
        self.draw_line(start, end, color)
    }

    pub fn save(&self, img_name: &str) -> ImageResult<()> {
        let mut tmp_dir = env::temp_dir();
        tmp_dir.push("crisscross");
        std::fs::create_dir_all(&tmp_dir)?;
        tmp_dir.push(img_name);

        eprintln!("{}", tmp_dir.display());
        self.image.save(tmp_dir)
    }

    pub fn log(&self, img_name: &str) -> ImageResult<()> {
        eprintln!("{}", img_name);
        self.image.save_with_format("/dev/stdout", ImageFormat::Png)
    }
}
