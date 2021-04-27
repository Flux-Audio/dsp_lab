//! All modules must implement these traits to be used in the framework macros.

use num::Float;

/// Every effect in an effect chain must implement this trait in order to use
/// the chain_exp! and chain_src! macros.
/// 
/// A process is a "stateful function", in other words it's a function that
/// takes a single input signal, and multiple parameters, and produces a single
/// output, with memory of previous states.
/// 
/// Processes take a generic type `T` which must implement the `Float` trait.
/// In essence this means that `T` must be one of:
/// + f32
/// + f64
/// + f80 or f128 if and when Rust is going to support them natively
pub trait Process<T>
where T: Float
{
    /// Feeds a sample as an input and produces an output, stepping time forward
    /// by a single time slot.
    fn step(&mut self, input: T) -> T;
}

/// Every source at the beginning of an effect chain must implement this
/// trait in order to use the chain_src! macro
///
/// A source is a process that takes no input. It's necessary to distinguish it
/// because the chain_exp! macro expects an expression as an input.
/// 
/// Sources take a generic type `T` which must implement the `Float` trait.
/// In essence this means that `T` must be one of:
/// + f32
/// + f64
/// + f80 or f128 if and when Rust is going to support them natively
pub trait Source<T>
where T: Float
{
    /// Generates an output, stepping time forward by a single time slot.
    fn step(&mut self) -> T;
}