use crate::{
    grid::Grid,
    ray::Ray,
    ray_iter::RayIter,
    position::TilePosition,
};

#[derive(Debug, Default, PartialEq)]
pub struct Crossing {
    pub valid: Option<TilePosition>,
    pub invalid: Option<TilePosition>,
}

pub struct TileRaycaster {
    grid: Grid,
}

impl TileRaycaster {
    #[must_use]
    pub const fn new(grid: Grid) -> Self {
        Self { grid }
    }

    #[must_use]
    pub fn cast_ray(&self, tp: TilePosition, angle: f32) -> RayIter {
        let intersections = Ray::new(self.grid.clone(), tp, angle);
        intersections.into_iter()
    }

    pub fn last_valid<P>(&self, tp: TilePosition, angle: f32, is_valid: P) -> Option<TilePosition>
    where
        P: FnMut(&TilePosition) -> bool,
    {
        self.cast_ray(tp, angle).take_while(is_valid).last()
    }

    pub fn crossing<P>(&self, tp: TilePosition, angle: f32, mut is_valid: P) -> Crossing
    where
        P: FnMut(&TilePosition) -> bool,
    {
        let mut iter = self.cast_ray(tp, angle).peekable();
        let mut previous = iter.next();

        match previous {
            None => Crossing::default(),
            Some(prev) if !is_valid(&prev) => Crossing::default(),
            Some(_) => {
                let (valid, invalid) = loop {
                    let next = iter.next();
                    match (previous, next) {
                        (Some(prev), Some(next)) => {
                            if is_valid(&next) {
                                previous = Some(next);
                                continue;
                            }
                            break (Some(prev), Some(next));
                        }
                        (Some(prev), None) => {
                            if is_valid(&prev) {
                                break (Some(prev), None);
                            }
                            break (None, None);
                        }
                        (None, None) => break (None, None),
                        #[allow(clippy::panic)]
                        (None, Some(_)) => {
                            panic!("(prev: None, next: Some(_)) should be impossible")
                        }
                    }
                };
                Crossing { valid, invalid }
            }
        }
    }
}
