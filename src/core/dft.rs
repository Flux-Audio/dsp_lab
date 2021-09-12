use num::complex::Complex;
use crate::core::RawRingBuffer;

pub struct SlidingDft {
    size: usize,
    input_buf: RawRingBuffer<2048>,
    frame_buf: Box<Complex<f64>>,
}

pub struct SlidingIdft {

}