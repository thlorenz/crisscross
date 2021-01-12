#[derive(Debug, Clone)]
pub struct Grid {
    pub cols: u32,
    pub rows: u32,
    pub tile_size: f32,
}

impl Grid {
    #[must_use]
    pub const fn new(cols: u32, rows: u32, tile_size: f32) -> Self {
        Self {
            cols,
            rows,
            tile_size,
        }
    }
}
