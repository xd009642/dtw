
/// Data type representing DTW alignments
pub struct Alignments {
    pub(crate) alignments: Vec<usize>,
}

impl Alignments {
    /// From a location (index) in the reference sequence get the 
    /// corresponding location it was observed in the provided sequence
    pub fn get_observed_location(index: usize) -> usize {
        unimplemented!();
    }
}

/// Crate for all dynamic time warping algorithms to implement
pub trait DynamicTimeWarping {
    /// TODO fill in types
    fn align<T>(reference: &[T], observed: &[T]) -> Alignments;
}
