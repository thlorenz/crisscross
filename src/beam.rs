use std::fmt;

use crate::{ray::Ray, ray_iter::RayIter, TilePosition};

#[derive(PartialEq)]
pub struct BeamIntersect(pub usize, pub TilePosition);

impl fmt::Debug for BeamIntersect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BeamIntersect({}, {:?})", self.0, self.1)
    }
}

pub struct Beam {
    rays: Vec<RayIter>,
    ray_origins: Vec<TilePosition>,
    tile_size: f32,
    intersects: Vec<Option<TilePosition>>,
}

impl Beam {
    pub(crate) fn new(tile_size: f32, rays: Vec<Ray>) -> Self {
        let ray_origins = rays.iter().map(|ray| ray.tp.clone()).collect();
        let mut rays: Vec<RayIter> = rays.into_iter().map(IntoIterator::into_iter).collect();

        let mut intersects = vec![None; rays.len()];
        for (idx, ray) in rays.iter_mut().enumerate() {
            // SAFETY we're enumerating rays and created intersects and ray_iters with same length above
            let intersect = unsafe { intersects.get_unchecked_mut(idx) };
            *intersect = ray.next();
        }
        Self {
            rays,
            ray_origins,
            tile_size,
            intersects,
        }
    }

    pub(crate) fn next_intersect(&mut self) -> Option<BeamIntersect> {
        let BeamIntersect(idx, tp) = self.closest_intersect()?;
        self.update_intersects(&tp);
        Some(BeamIntersect(idx, tp))
    }

    fn update_intersects(&mut self, intersect_tp: &TilePosition) {
        let idxs_with_identical_xy: Vec<usize> = self
            .intersects
            .iter()
            .enumerate()
            .filter_map(|(idx, tp)| {
                if tp.as_ref()?.is_same_tile(intersect_tp) {
                    Some(idx)
                } else {
                    None
                }
            })
            .collect();

        for idx in idxs_with_identical_xy {
            // SAFETY we got indexes by iterating intersects above and know that rays have
            // same length (see `Beam::new`).
            let slot = unsafe { self.intersects.get_unchecked_mut(idx) };
            let ray = unsafe { self.rays.get_unchecked_mut(idx) };
            *slot = ray.next();
        }
    }

    fn closest_intersect(&self) -> Option<BeamIntersect> {
        #[allow(clippy::filter_map, clippy::unwrap_used)]
        let (idx, tp, _) = self
            .intersects
            .iter()
            .enumerate()
            .filter(|(_, tp)| tp.is_some())
            .map(|(idx, tp)| {
                // SAFETY we setup ray_origins and intersects by mapping over rays
                // therefore they have the same amount of items
                let orig = unsafe { self.ray_origins.get_unchecked(idx) };
                (idx, tp.as_ref().unwrap(), orig)
            })
            .min_by(|(_, tp1, orig1), (_, tp2, orig2)| {
                let dist1 = orig1.distance_global(*tp1, self.tile_size);
                let dist2 = orig2.distance_global(*tp2, self.tile_size);
                dist1.partial_cmp(&dist2).unwrap()
            })?;
        Some(BeamIntersect(idx, tp.clone()))
    }
}
