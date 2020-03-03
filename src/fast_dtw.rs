use crate::simple::*;
use crate::types::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct FastDtw {
    radius: usize,
}

impl FastDtw {
    pub fn new(radius: usize) -> Self {
        Self { radius }
    }

    fn align_by_iter<It, T, F>(&self, reference: It, observed: It, distance: F) -> Alignments
    where
        It: Iterator<Item = T> + Clone,
        F: Fn(T, T) -> f64,
    {
        unimplemented!();
    }
}

impl DynamicTimeWarping for FastDtw {
    /// Runs alignment for LB Keogh DTW algorithm
    /// Currently only works on sequences of the same length
    fn align<T, F>(&self, reference: &[T], observed: &[T], distance: F) -> Alignments
    where
        F: Fn(&T, &T) -> f64,
    {
        let simple = SimpleDtw::new().build();
        let min_size = self.radius + 2;
        if reference.len() <= min_size || observed.len() <= min_size {
            simple.align(reference, observed, distance)
        } else {
            let shrunk_r = reference.iter().step_by(2);
            let shrunk_o = reference.iter().step_by(2);
            let low_res = self.align_by_iter(shrunk_r, shrunk_o, distance);
            unimplemented!()
        }
    }
}
