//! All modules must implement these traits to be used in the framework macros.

use num::Float;

/// Every effect in an effect chain must implement this trait in order to use
/// the chain! macro.
/// 
/// A process is a "stateful function", in other words it's a function that
/// takes a single input signal, and multiple parameters, and produces a single
/// output, with memory of previous states.
/// 
/// Processes take a generic type `T` which must implement the `Float` trait.
/// In essence this means that `T` must be one of:
/// + f32
/// + f64
pub trait Process<T>
where T: Float
{
    /// Feeds a sample as an input and produces an output, stepping time forward
    /// by a single time slot.
    fn step(&mut self, input: T) -> T;
}