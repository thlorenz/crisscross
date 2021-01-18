use crate::beam::{Beam, BeamIntersect};

pub struct BeamIter {
    beam: Beam,
}

impl Beam {
    const fn iter(self) -> BeamIter {
        BeamIter { beam: self }
    }
}

impl Iterator for BeamIter {
    type Item = BeamIntersect;

    fn next(&mut self) -> Option<Self::Item> {
        self.beam.next_intersect()
    }
}

impl IntoIterator for Beam {
    type Item = BeamIntersect;
    type IntoIter = BeamIter;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
