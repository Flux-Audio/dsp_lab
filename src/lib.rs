/*! 
DSP_lab is a framework for developing signal processing effects, or building
effect chains from built-in DSP_lab effects.

Each effect in the chain implements an abstract "Process" trait. This allows to chain
different effects to form complex signal chains. For example, one can emulate an
analog mastering chain by chaining together various built-in effects, like transformers,
tube pre-amps and compressors.

Importantly, the "Process" abstraction also covers generators, which essentially
are taking a stream of "nothing" as inputs and producing an output. Everything
within the framework is either a process or a chain. A process can also contain
instances of chains, which themselves contain processes.
*/

pub mod traits;
pub mod utils;
pub mod core;
pub mod effects;
pub mod emulation;

/// This macro is used to build signal chains.
/// 
/// It take an expression, followed by one or more identifiers which must be
/// instances of a "Process" trait, separated by an arrow `=>`. The identifiers'
/// `step()` function are then composed in the order of the arrow, and the result
/// of the expression is fed as an argument to the composed function.
/// 
/// # Examples
/// General usage:
/// ```
/// # #[macro_use] extern crate dsp_lab;
/// use dsp_lab::core::EmptyProcess;
/// use dsp_lab::traits::Process;
/// 
/// # fn main(){
/// let p1 = EmptyProcess{};
/// let p2 = EmptyProcess{};
/// let ch1 = chain!{1.0 => p1 => p2};
/// 
/// assert_eq!(ch1, 1.0);
/// assert_eq!(ch1, p2.step(p1.step(1.0)));
/// # }
/// ```
/// 
/// Branching and combining chains:
/// ```
/// # #[macro_use] extern crate dsp_lab;
/// # use dsp_lab::core::EmptyProcess;
/// # use dsp_lab::traits::Process;
/// 
/// struct AddOne {}
/// /* impl omitted */
/// # impl Process<f32> for AddOne {
/// #    fn step(&self, input: f32) -> f32 { input + 1.0 }
/// # }
/// 
/// # fn main(){
/// let p1 = AddOne{};
/// let ch1 = chain!{1.0 => p1};
/// 
/// // Branching of ch1 into two chains ch2 and ch3:
/// let p2 = AddOne{};
/// let p3 = EmptyProcess{};
/// let ch2 = chain!{ch1 => p2};
/// let ch3 = chain!{ch1 => p3};
/// 
/// // Mergin ch2 and ch3 into a signle chain c4
/// let p4 = AddOne{};
/// let ch4 = chain!{ch2 * ch3 => p4};
/// 
/// assert_eq!(ch4, 7.0);
/// # }
/// ```
#[macro_export]
macro_rules! chain {
    // Base case: single function call
    { $arg:expr => $p:ident } => {
        $p.step($arg);
    };

    // Recursive case: chaining the output of the first function call in the
    // chain with the rest of the chain.
    { $arg:expr => $p:ident => $($tokens:tt)* } => {{
        chain!($p.step($arg) => $($tokens)*)
    }};
}

// Non-documented tests
#[cfg(test)]
mod tests {
    #[test]
    fn placeholder() {
        assert!(true);
    }
}