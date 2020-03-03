use ndarray::{ArrayBase, Data, Ix2};
use std::f64::INFINITY;

/// Data type representing DTW alignments
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct Alignments {
    /// Warp path to go to the reference sequence from the observed
    pub(crate) warp_path: Vec<(usize, usize)>,
    /// Sum of distances for the path
    pub warped_path_distance: f64,
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

impl<S> From<ArrayBase<S, Ix2>> for Alignments
where
    S: Data<Elem = f64>,
{
    fn from(cost_mat: ArrayBase<S, Ix2>) -> Self {
        let mut result = Alignments::default();
        let (row, col) = cost_mat.dim();
        result.warp_path.reserve(std::cmp::max(row, col));
        let mut row = row as isize;
        let mut col = col as isize;
        row -= 1;
        col -= 1;
        let get_cost = |r, c| {
            if r < 0 || c < 0 {
                INFINITY
            } else {
                cost_mat[[r as usize, c as usize]]
            }
        };
        while !(row == 0 && col == 0) {
            result.warp_path.push((row as usize, col as usize));
            let c0 = get_cost(row - 1, col);
            let c1 = get_cost(row - 1, col - 1);
            let c2 = get_cost(row, col - 1);
            if c0 < c1 && c0 < c2 {
                row -= 1;
                result.warped_path_distance += c0;
            } else if c1 < c2 {
                row -= 1;
                col -= 1;
                result.warped_path_distance += c1;
            } else {
                col -= 1;
                result.warped_path_distance += c2;
            }
        }
        result.warp_path.push((0, 0));
        result.warp_path.reverse();
        result
    }
}

/// Crate for all dynamic time warping algorithms to implement
pub trait DynamicTimeWarping {
    /// TODO fill in types
    fn align<T, F>(&self, reference: &[T], observed: &[T], distance: F) -> Alignments
    where
        F: Fn(&T, &T) -> f64;
}
