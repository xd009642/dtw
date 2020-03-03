use crate::types::*;
use float_ord::FloatOrd;
use ndarray::prelude::*;
use std::cmp::{max, min};
use std::f64::INFINITY;

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SimpleDtwBuilder {
    window: Option<usize>,
}

impl SimpleDtwBuilder {
    pub fn window(mut self, len: usize) -> Self {
        self.window = Some(len);
        self
    }

    pub fn build(self) -> SimpleDtw {
        SimpleDtw {
            window: self.window,
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SimpleDtw {
    window: Option<usize>,
}

impl Default for SimpleDtw {
    fn default() -> Self {
        Self::new().build()
    }
}

impl SimpleDtw {
    pub fn new() -> SimpleDtwBuilder {
        Default::default()
    }
}

impl DynamicTimeWarping for SimpleDtw {
    /// Runs alignment for LB Keogh DTW algorithm
    /// Currently only works on sequences of the same length
    fn align<T, F>(&self, reference: &[T], observed: &[T], distance: F) -> Alignments
    where
        F: Fn(&T, &T) -> f64,
    {
        if reference.is_empty() || observed.is_empty() {
            return Default::default();
        }
        let mut cost = Array::from_elem((reference.len(), observed.len()), INFINITY);
        cost[[0, 0]] = distance(&reference[0], &observed[0]);
        if let Some(w) = self.window {
            for i in 1..reference.len() {
                let lower = max(1, i as isize - w as isize) as usize;
                let upper = min(observed.len(), i + w);
                for j in lower..upper {
                    cost[[i, j]] = 0.0;
                }
            }
        }
        for i in 1..reference.len() {
            let (lower, upper) = match self.window {
                Some(w) => {
                    let lower = max(1, i as isize - w as isize) as usize;
                    let upper = min(observed.len(), i + w);
                    (lower, upper)
                }
                None => (1, observed.len()),
            };
            for j in lower..upper {
                let temp_cost = distance(&reference[i], &observed[j]);
                let mut min_cost = FloatOrd(INFINITY);
                if i > 0 {
                    min_cost = min(min_cost, FloatOrd(cost[[i - 1, j]]));
                }
                if j > 0 {
                    min_cost = min(min_cost, FloatOrd(cost[[i, j - 1]]));
                }
                if i > 0 && j > 0 {
                    min_cost = min(min_cost, FloatOrd(cost[[i - 1, j - 1]]));
                }
                let temp_cost = temp_cost + min_cost.0;
                cost[[i, j]] = temp_cost;
            }
        }
        cost.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::distances::*;

    #[test]
    fn easy_test() {
        let reference = vec![0, 1, 1, 2, 3, 2, 1];
        let observed = vec![1, 1, 2, 3, 2, 0];

        let dtw = SimpleDtw::default();

        let alignments = dtw.align(&reference, &observed, euclidean);
        let expected_path = vec![(0, 0), (1, 1), (2, 1), (3, 2), (4, 3), (5, 4), (6, 5)];
        assert_eq!(alignments.warp_path, expected_path);
    }
}
