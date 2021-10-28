//! This file contains standard implementations for some of the functions in the 
//! math module which require multiple implementations. These make use of intrinsics
//! for float operations, like trigonometry, square roots, etc...

pub fn impl_fast_sigmoid(x: f64) -> f64 { x / (1.0 + x * x).sqrt() }