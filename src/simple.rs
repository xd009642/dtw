use crate::types::*;
use float_ord::FloatOrd;
use ndarray::prelude::*;
use std::cmp::{max, min};
use std::f64::INFINITY;

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SimpleDtw {
    window: usize
}

impl SimpleDtw {
    pub fn new(window: usize) -> Self {
        Self {
            window
        }
    }
}

impl DynamicTimeWarping for SimpleDtw {
    /// Runs alignment for LB Keogh DTW algorithm
    /// Currently only works on sequences of the same length
    fn align<T, F>(&self, reference: &[T], observed: &[T], distance: F) -> Alignments
    where
        F: Fn(&T, &T) -> f64,
    {
        let mut cost = Array::from_shape_fn((reference.len(), observed.len()), |(r, c)| {
            
            if (c==0 && r== 0) || 
                ( c >= max(1, r - self.window) && c < min(observed.len(), r + self.window)) 
            {
                0.0f64
            } else {
                INFINITY
            }
        });

        for i in 1..reference.len() {
            for j in max(1, i-self.window)..min(observed.len(), i+self.window) {
                let temp_cost = distance(&reference[i], &observed[i]);
                let c0 = FloatOrd(cost[[i-1, j]]);
                let c1 = FloatOrd(cost[[i, j-1]]);
                let c2 = FloatOrd(cost[[i-1, j-1]]);
                let temp_cost = temp_cost + min(c0, min(c1, c2)).0;
                cost[[i, j]] = temp_cost;
            }
        }
        cost.into()
    }
}

