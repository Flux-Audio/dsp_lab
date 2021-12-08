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


pub mod traits;/*
pub mod utils;
pub mod core;
pub mod effects;
pub mod virtual_analog;
// pub mod physical_modelling;
pub mod shared_enums; */
pub mod core;
pub mod types;

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
/// # impl Process<f64> for AddOne {
/// #    fn step(&mut self, input: f64) -> f64 { input + 1.0 }
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
#[deprecated(since="0.2.0", note="Deprecated since macros cannot use the self
keyword. Use ProcessChain instead.")]
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

    // Alternate base case: calling a process on self
    { $arg:expr => self.$p:ident } => {
        self.$p.step($arg);
    };

    // Alternate recursive case: calling a process on self
    /* TODO: self in macros is not supported yet...
    { $arg:expr => self.$p:ident => $($tokens:tt)* } => {{
        chain!(self.$p.step($arg) => $($tokens)*)
    }};
    */
}


/// This macro adds syntactic sugar for chaining sources (processes that take no
/// input) into chains of processes.
/// 
/// # Examples
/// General usage:
/// ```
/// # #[macro_use] extern crate dsp_lab;
/// use dsp_lab::core::{EmptyProcess, EmptySource};
/// use dsp_lab::traits::{Process, Source};
/// 
/// # fn main(){
/// let mut s = EmptySource{};
/// let mut p = EmptyProcess{};
/// let ch = chain_src!{s => p};
/// 
/// assert_eq!(ch, 1.0);
/// assert_eq!(ch, p.step(s.step()));
/// # }
/// ```
#[macro_export]
#[deprecated(since="0.2.0", note="Deprecated since macros cannot use the self
keyword. Use SourceChain instead.")]
macro_rules! chain_src {
    // Base case: parse source, and pass expression to chain!
    { $src:ident => $($tokens:tt)* } => {
        chain!($src.step() => $($tokens)*);
    };

    // Alternate case: calling a source on self
    /* TODO: self in macros is not supported yet...
    { self.$src:ident => $($tokens:tt)* } => {
        chain!(self.$src.step() => $($tokens)*);
    }
    */
}


/*
// Non-documented tests
#[cfg(test)]
mod tests {

    #[test]
    fn unit_test_process_chain() {
        use crate::traits::{ProcessChain};
        use crate::core::{EmptyProcess};
        let mut p1 = EmptyProcess{};
        let mut p2 = EmptyProcess{};
        let res = ProcessChain::new(1.0)
            .pipe(&mut p1)
            .pipe(&mut p2)
            .consume();
        assert!(res == 1.0);
    }

    #[test]
    fn unit_test_random_core() {
        use crate::core::chaos::RandomCore;
        let mut rng1 = RandomCore::new();
        let mut rng2 = RandomCore::new();
        rng1.reseed(11_u8);
        rng2.reseed(17_u8);
        assert!(rng1.next() != rng2.next());
    }

    #[test]
    fn unit_test_random_coin() {
        use crate::core::chaos::RandomCoin;
        use crate::traits::Source;
        let mut coin = RandomCoin::new(11_u8);
        coin.p = 0.0;
        for _ in 0..100 { assert!(coin.step() == 0.0); }
        coin.p = 1.0;
        for _ in 0..100 { assert!(coin.step() == 1.0); }
    }

    # [test]
    fn unit_test_random_toggle() {
        use crate::core::chaos::RandomToggle;
        use crate::traits::Source;
        let mut toggle = RandomToggle::new(11_u8);
        for _ in 0..10 { toggle.step(); }
        toggle.p_up = 0.0;
        toggle.p_down = 1.0;
        toggle.step();
        assert!(toggle.step() == 0.0);
        toggle.p_up = 0.25;
        toggle.p_down = 0.25;
        for _ in 0..10 { toggle.step(); }
        toggle.p_up = 1.0;
        toggle.p_down = 0.0;
        toggle.step();
        assert!(toggle.step() == 1.0);
    }

    #[test]
    fn unit_test_noise_white() {
        use crate::core::chaos::NoiseWhite;
        use crate::traits::Source;
        let mut nse = NoiseWhite::new(11);
        assert!(nse.step() != nse.step());  
        let mut acc: f64 = 0.0;
        for _ in 0..10000 {
            let sample = nse.step();
            acc += sample;
            assert!(sample < 1.0);
        }
        acc /= 10000.0;
        assert!(acc > 0.45 && acc < 0.55);
    }

    /* FIXME: dunno what's up with these
    #[test]
    fn unit_test_ramp_core() {
        use crate::core::osc::RampCore;
        use crate::traits::Source;
        use std::f64::consts;
        let mut ramp = RampCore::new();
        ramp.set_freq(1000.0);
        for _ in 0..2001 {
            assert!(ramp.step() <= consts::TAU);
        }
        assert!(ramp.step() <= 0.01);
    }

    #[test]
    fn unit_test_par_osc() {
        use crate::core::osc::ParOsc;
        use crate::traits::Source;
        let mut osc = ParOsc::new(0.0, 1000.0);
        osc.set_freq(1.0);
        for _ in 0..2001 {
            assert!(osc.step() <= 1.0);
        }
        assert!(osc.step() <= 0.01);
    }

    #[test]
    fn unit_test_asym_tri_osc() {
        use crate::core::osc::AsymTriOsc;
        use crate::traits::Source;
        let mut osc = AsymTriOsc::new(0.0, 1000.0);
        osc.set_freq(1.0);
        for _ in 0..2001 {
            assert!(osc.step() <= 1.0);
        }
        assert!(osc.step() <= 0.01);
    }*/

    #[test]
    fn unit_test_raw_ring_buffer() {
        use crate::core::RawRingBuffer;
        let mut buf = RawRingBuffer::<4>::new();
        buf.push(1.0);
        buf.push(2.0);
        buf.push(3.0);
        buf.push(4.0);
        buf.push(5.0);
        assert!(buf[0] == 5.0);
        assert!(buf[1] == 4.0);
        assert!(buf[2] == 3.0);
        assert!(buf[3] == 2.0);
    }

    #[test]
    fn stress_test_raw_ring_buffer() {
        use crate::core::RawRingBuffer;
        use crate::core::chaos::NoiseWhite;
        use crate::traits::Source;
        let mut buf = RawRingBuffer::<32768>::new();
        let mut noise = NoiseWhite::new(4);

        for i in 0..1000000 {
            let idx = (i*11 + 73) % 32768;
            buf.push(noise.step());
            assert!(buf.get(idx) == buf[idx]);
            assert!(buf.get(idx) <= 1.0 && buf.get(idx) >= -1.0);
        }
    }

    #[test]
    fn unit_test_safe_raw_ring_buffer() {
        use crate::core::SafeRawRingBuffer;
        assert!(SafeRawRingBuffer::<6>::new().is_none());
        let mut buf = SafeRawRingBuffer::<4>::new().unwrap();
        buf.push(1.0);
        buf.push(2.0);
        buf.push(3.0);
        buf.push(4.0);
        buf.push(5.0);
        assert!(buf.get(0) == Some(5.0));
        assert!(buf.get(1) == Some(4.0));
        assert!(buf.get(2) == Some(3.0));
        assert!(buf.get(3) == Some(2.0));
        assert!(buf.get(4).is_none());
    }

    #[test]
    fn unit_test_dense_diffuser() {
        use crate::core::reverb::DenseFirDiffuser;
        use crate::traits::Process;
        let mut diff = DenseFirDiffuser::new();
        diff.size = 0.2;
        diff.step(1.0);
        let mut it_probably_works: bool = false;
        for _ in 0..1000 {
            let res = diff.step(0.0);
            if res != 0.0 {
                it_probably_works = true;
            }
        }
        assert!(it_probably_works);

        diff.size = 1.0;
        diff.step(1.0);
        it_probably_works = false;
        for _ in 0..1000 {
            let res = diff.step(0.0);
            if res != 0.0 {
                it_probably_works = true;
            }
        }
        assert!(it_probably_works);

        diff.size = 10.0;
        diff.step(1.0);
    }

    /*
    NOTE: this test is quite slow, uncomment if you need it.
    #[test]
    fn stress_test_dense_diffuser() {
        use crate::core::reverb::DenseFirDiffuser;
        use crate::traits::Process;
        use crate::traits::Source;
        use crate::core::chaos::NoiseWhite;
        let mut diff = DenseFirDiffuser::new();
        let mut noise = NoiseWhite::new(5);
        diff.size = 1.0;

        for i in 0..100000 {
            diff.size = ((i + 73) % 89) as f64 / 89.0;
            diff.step(noise.step());
        }
    }*/

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn white_box_test_raw_ring_buffer_1() {
        use crate::core::RawRingBuffer;
        let ring = RawRingBuffer::<0>::new();
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn white_box_test_raw_ring_buffer_2() {
        use crate::core::RawRingBuffer;
        let ring = RawRingBuffer::<5>::new();
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn white_box_test_raw_ring_buffer_3() {
        use crate::core::RawRingBuffer;
        let mut ring = RawRingBuffer::<4>::new();
        ring.get(4);
    }

    #[test]
    fn white_box_test_raw_ring_buffer_4() {
        use crate::core::RawRingBuffer;
        let mut ring = RawRingBuffer::<4>::new();
        ring.get(3);
        ring.push(0.0);
        ring.get(3);
        ring.push(0.0);
        ring.get(3);
        ring.push(0.0);
        ring.get(3);
        ring.push(0.0);
        ring.get(3);
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn white_box_test_raw_ring_buffer_5() {
        use crate::core::RawRingBuffer;
        let ring = RawRingBuffer::<4>::new();
        ring[4];
    }

    #[test]
    fn white_box_test_raw_ring_buffer_6() {
        use crate::core::RawRingBuffer;
        let mut ring = RawRingBuffer::<4>::new();
        ring[3];
        ring.push(0.0);
        ring[3];
        ring.push(0.0);
        ring[3];
        ring.push(0.0);
        ring[3];
        ring.push(0.0);
        ring[3];
    }

    #[test]
    fn unit_test_delay_line() {
        use crate::core::delay::{DelayLine};
        use crate::traits::Process;
        use crate::shared_enums::{ScaleMethod, InterpMethod};
        let mut delay = DelayLine::new();
        delay.mix_mode = ScaleMethod::Unity;
        delay.interp_mode = InterpMethod::Truncate;
        delay.add_head(100.0, 1.0);
        delay.add_head(200.0, -1.0);

        for _ in 0..100000 {
            delay.step(1.0);
        }

        delay.interp_mode = InterpMethod::NearestNeighbor;
        delay.mix_mode = ScaleMethod::Perceptual;

        for _ in 0..100000 {
            delay.step(1.0);
        }

        delay.interp_mode = InterpMethod::Linear;
        delay.mix_mode = ScaleMethod::Off;

        for _ in 0..100000 {
            delay.step(1.0);
        }

        delay.interp_mode = InterpMethod::Quadratic;

        for _ in 0..100000 {
            delay.step(1.0);
        }
    }
}
*/
