use crate::types::*;

#[derive(Clone, Debug, Default)]
pub struct LbKeoghDtwBuilder {
    window_size: Option<usize>,
    norm: Option<usize>,
    lower_envelope: Option<usize>,
    force_symmetry: Option<bool>,
    error_check: Option<bool>,
}

#[derive(Clone, Debug, Default)]
pub struct LbKeoghDtw {
    window_size: usize,
    norm: usize,
    lower_envelope: usize,
    force_symmetry: bool,
    error_check: bool,
}

impl LbKeoghDtw {
    pub fn new() -> LbKeoghDtwBuilder {
        LbKeoghDtwBuilder::default()
    }
}

impl DynamicTimeWarping for LbKeoghDtw {
    /// Runs alignment for LB Keogh DTW algorithm
    /// Currently only works on sequences of the same length
    fn align<T, F>(&self, reference: &[T], observed: &[T], distance: F) -> Alignments
    where
        F: Fn(&T, &T) -> f64,
    {
        assert_eq!(reference.len(), observed.len());
        unimplemented!()
    }
}
