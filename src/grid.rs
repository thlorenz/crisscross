#[derive(Debug, Clone)]
pub struct Grid {
    pub cols: u32,
    pub rows: u32,
    pub tile_size: f32,
    pub width: f64,
    pub height: f64,
}

impl Grid {
    #[must_use]
    pub fn new(cols: u32, rows: u32, tile_size: f32) -> Self {
        let width = f64::from(cols) * f64::from(tile_size);
        let height = f64::from(rows) * f64::from(tile_size);
        Self {
            cols,
            rows,
            tile_size,
            width,
            height,
        }
    }
}
