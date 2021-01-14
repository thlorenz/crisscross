use crate::{ray::Ray, position::TilePosition};

pub struct RayIter {
    intersections: Ray,
    last_intersect: Option<(u32, u32)>,
}

impl Ray {
    const fn iter(self) -> RayIter {
        RayIter {
            intersections: self,
            last_intersect: None,
        }
    }
}

impl Iterator for RayIter {
    type Item = TilePosition;

    fn next(&mut self) -> Option<Self::Item> {
        let next_intersect = self.intersections.next_intersect()?;

        // Ensure that we don't emit the same tile position twice which could happen if
        // x and y intersections are the same, i.e. for a 45 deg angle
        let TilePosition { x, y, .. } = next_intersect;
        let next_intersect_x_y = Some((x, y));

        if next_intersect_x_y == self.last_intersect {
            self.next()
        } else {
            self.last_intersect = next_intersect_x_y;
            Some(next_intersect)
        }
    }
}

impl IntoIterator for Ray {
    type Item = TilePosition;
    type IntoIter = RayIter;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
