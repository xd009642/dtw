use ndarray::{ArrayBase, Data, Ix2};

/// Data type representing DTW alignments
pub struct Alignments {
    pub(crate) warp_path: Vec<(usize, usize)>,
    pub warped_path_distance: Option<f64>,
}

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Segment {
    pub start: usize,
    pub len: usize,
}

impl Alignments {
    /// From a location (index) in the reference sequence get the
    /// corresponding location it was observed in the provided sequence
    pub fn get_observed_location(index: usize) -> Segment {
        unimplemented!();
    }
}

impl<S> From<ArrayBase<S, Ix2>> for Alignments where S: Data<Elem=f64> {
    fn from(cost_mat: ArrayBase<S, Ix2>) -> Self {
        unimplemented!();
    }
}

/// Crate for all dynamic time warping algorithms to implement
pub trait DynamicTimeWarping {
    /// TODO fill in types
    fn align<T, F>(&self, reference: &[T], observed: &[T], distance: F) -> Alignments
    where
        F: Fn(&T, &T) -> f64;
}
