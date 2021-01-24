use crate::{
    beam::Beam, beam_iter::BeamIter, grid::Grid, position::TilePosition, ray::Ray,
    ray_iter::RayIter, rays::rays_from, AngleRad, BeamIntersect,
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

    pub const fn grid(&self) -> &Grid {
        &self.grid
    }

    #[must_use]
    pub fn cast_ray<T: Into<AngleRad>>(&self, tp: &TilePosition, angle: T) -> RayIter {
        let intersections = Ray::new(self.grid.clone(), tp.clone(), angle);
        intersections.into_iter()
    }

    pub fn cast_beam<T: Into<AngleRad>>(
        &self,
        beam_center: &TilePosition,
        beam_width: f32,
        angle: T,
    ) -> BeamIter {
        let rays = rays_from(beam_center, &self.grid, beam_width, &angle.into());
        Beam::new(self.grid.tile_size, rays).into_iter()
    }

    pub fn last_valid<P, T: Into<AngleRad>>(
        &self,
        tp: &TilePosition,
        angle: T,
        is_valid: P,
    ) -> Option<TilePosition>
    where
        P: FnMut(&TilePosition) -> bool,
    {
        self.cast_ray(tp, angle).take_while(is_valid).last()
    }

    pub fn beam_last_valid<P, T: Into<AngleRad>>(
        &self,
        beam_center: &TilePosition,
        beam_width: f32,
        angle: T,
        is_valid: P,
    ) -> Option<BeamIntersect>
    where
        P: FnMut(&BeamIntersect) -> bool,
    {
        self.cast_beam(beam_center, beam_width, angle)
            .take_while(is_valid)
            .last()
    }

    pub fn crossing<P, T: Into<AngleRad>>(
        &self,
        tp: &TilePosition,
        angle: T,
        mut is_valid: P,
    ) -> Crossing
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
