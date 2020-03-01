use crate::types::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct FastDtw {
    radius: f64
}

impl FastDtw {
    pub fn new(radius: f64) -> Self {
        Self {
            radius
        }
    }
}

impl DynamicTimeWarping for FastDtw {
    /// Runs alignment for LB Keogh DTW algorithm
    /// Currently only works on sequences of the same length
    fn align<T, F>(&self, reference: &[T], observed: &[T], distance: F) -> Alignments
    where
        F: Fn(&T, &T) -> f64,
    {
        unimplemented!()
    }
}
