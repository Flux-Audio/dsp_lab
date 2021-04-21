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

#![feature(is_subnormal)]
#![feature(test)]

extern crate test;
use test::{Bencher, black_box};

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
/// let mut p1 = EmptyProcess{};
/// let mut p2 = EmptyProcess{};
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
/// #    fn step(&mut self, input: f32) -> f32 { input + 1.0 }
/// # }
/// 
/// # fn main(){
/// let mut p1 = AddOne{};
/// let ch1 = chain!{1.0 => p1};
/// 
/// // Branching of ch1 into two chains ch2 and ch3:
/// let mut p2 = AddOne{};
/// let mut p3 = EmptyProcess{};
/// let ch2 = chain!{ch1 => p2};
/// let ch3 = chain!{ch1 => p3};
/// 
/// // Mergin ch2 and ch3 into a signle chain c4
/// let mut p4 = AddOne{};
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
    fn test_placeholder() {
        assert!(true);
    }

    #[test]
    fn test_fast_powf() {
        use crate::utils::math::fast_powf;
        assert!( (fast_powf(2.1,  2.1) - 2.1_f32.powf( 2.1)).abs() < 0.002  );
        assert!( (fast_powf(2.1,  4.1) - 2.1_f32.powf( 4.1)).abs() < 0.07   );
        assert!( (fast_powf(2.1, -2.1) - 2.1_f32.powf(-2.1)).abs() < 0.0001 );
    }

    #[test]
    fn test_fast_tanh() {
        use crate::utils::math::fast_tanh;
        println!("{}", fast_tanh(0.75));
        assert!( (fast_tanh( 0.75) - 0.75_f32.tanh()).abs() < 0.0002 );
        assert!( (fast_tanh(-0.75) + 0.75_f32.tanh()).abs() < 0.0002 );
        assert!( fast_tanh(0.05) == 0.05 );
        assert!( fast_tanh(5.1) == 1.0 );
    }

    // === STD POWF vs. FAST_POWF ==============================================
    // In these tests base and exponent are kept within ranges that are common
    // for signal processing applications, i.e. 
    // 0 <= base <= 2, and 
    // 0 <= exponent <= 10
    #[bench]
    fn bench_std_powf(b: &mut test::Bencher) {
        let range = test::black_box(1000);
        let mut _a = 0.0;   // garbage variable to prevent optimization

        b.iter(|| {
            for b in 0..range {
                for x in 0..range {
                    _a += (b as f32 * 0.002).powf(x as f32 * 0.01);
                }
            }

            // prevent optimizing away entire test body, by black-boxing return
            test::black_box(_a)
        });
    }

    #[bench]
    fn bench_fast_powf(b: &mut test::Bencher) {
        use crate::utils::math::fast_powf;
        let range = test::black_box(1000);
        let mut _a = 0.0;   // garbage variable to prevent optimization

        b.iter(|| {
            for b in 0..range {
                for x in 0..range {
                    _a += fast_powf(b as f32 * 0.002, x as f32 * 0.01);
                }
            }

            // prevent optimizing away entire test body, by black-boxing return
            test::black_box(_a)
        });
    }


    // === STD TANH vs. FAST TANH ==============================================
    #[bench]
    fn bench_std_tanh(b: &mut test::Bencher) {
        let range = test::black_box(1000);
        let mut _a = 0.0;   // garbage variable to prevent optimization

        b.iter(|| {
            for x in 0..range {
                _a += (x as f32 * 0.005).tanh();
            }

            // prevent optimizing away entire test body, by black-boxing return
            test::black_box(_a)
        });
    }

    #[bench]
    fn bench_fast_tanh(b: &mut test::Bencher) {
        use crate::utils::math::fast_tanh;
        let range = test::black_box(1000);
        let mut _a = 0.0;   // garbage variable to prevent optimization

        b.iter(|| {
            for x in 0..range {
                _a += fast_tanh(x as f32 * 0.005);
            }

            // prevent optimizing away entire test body, by black-boxing return
            test::black_box(_a)
        });
    }


    // === HYSTERESIS ==========================================================
    #[bench]
    fn bench_hyst_fast(b: &mut test::Bencher) {
        use crate::emulation::Hysteresis;
        use crate::traits::Process;
        let mut hyst = Hysteresis::new();
        let range = test::black_box(1000);
        let mut _a = 0.0;   // garbage variable to prevent optimization

        b.iter(|| {
            for x in 0..range {
                _a += hyst.step(x as f32 * 0.001);
            }

            // prevent optimizing away entire test body, by black-boxing return
            test::black_box(_a)
        });
    }

    #[bench]
    fn bench_hyst_hq(b: &mut test::Bencher) {
        use crate::emulation::Hysteresis;
        use crate::traits::Process;
        let mut hyst = Hysteresis::new();
        hyst.fast = false;
        let range = test::black_box(1000);
        let mut _a = 0.0;   // garbage variable to prevent optimization

        b.iter(|| {
            for x in 0..range {
                _a += hyst.step(x as f32 * 0.001);
            }

            // prevent optimizing away entire test body, by black-boxing return
            test::black_box(_a)
        });
    }
}