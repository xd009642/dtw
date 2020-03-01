//! This crate is intended to provide a number of algorithms for discrete time
//! warping that make various tradeoffs between the accuracy of the results and
//! runtime performance.

pub mod fast_dtw;
pub mod lb_keogh;
pub mod simple;
pub mod types;

pub mod prelude {
    pub use crate::fast_dtw::*;
    pub use crate::lb_keogh::*;
    pub use crate::simple::*;
    pub use crate::types::*;
}
