use std::f32::{
    consts::{PI, TAU},
    EPSILON,
};

const DEG_90: f32 = PI * 0.5;
const DEG_270: f32 = PI * 1.5;

#[derive(Clone)]
pub struct AngleRad(pub f32);

impl AngleRad {
    pub fn clamp(&self) -> Self {
        // Clamp angle to 0.0..TAU
        let angle = match self.0 {
            x if x >= TAU => x - TAU,
            x if x < 0.0 => x + TAU,
            x => x,
        };
        Self(angle)
    }

    pub fn perpendicular(&self) -> Self {
        Self(self.0 - PI)
    }

    pub fn sin(&self) -> f32 {
        self.0.sin()
    }
    pub fn cos(&self) -> f32 {
        self.0.cos()
    }
}

impl From<f32> for AngleRad {
    fn from(radians: f32) -> Self {
        Self(radians)
    }
}

#[derive(Debug, PartialEq)]
pub enum DirectionX {
    Left,
    Right,
    Parallel,
}
#[derive(Debug, PartialEq)]
pub enum DirectionY {
    Up,
    Down,
    Parallel,
}

impl From<&AngleRad> for DirectionX {
    fn from(angle: &AngleRad) -> Self {
        match angle.0 {
            x if (x - DEG_90).abs() < EPSILON || (x - DEG_270).abs() < EPSILON => {
                Self::Parallel
            }
            x if !(DEG_90..=DEG_270).contains(&x) => Self::Right,
            _ => Self::Left,
        }
    }
}

impl From<&AngleRad> for DirectionY {
    fn from(angle: &AngleRad) -> Self {
        match angle.0 {
            x if x.abs() < EPSILON || (x - PI).abs() < EPSILON => Self::Parallel,
            x if x < PI => Self::Up,
            _ => Self::Down,
        }
    }
}
