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
    fn align<T>(reference: &[T], observed: &[T]) -> Alignments {
        unimplemented!()
    }
}
