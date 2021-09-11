/// Used to select sample interpolation method.
/// 
/// - Truncate: no interpolation, fastest
/// - NearestNeighbor: select the closest index to the specified time, fast
/// - Linear: you need at least this to remove artifacts when modulating time
///   however it will distort a tiny bit, pretty much as fast (if not faster) than
///   nearest neighbor.
/// - Quadratic: less distortion than linear, slowest
pub enum InterpMethod {
    Truncate,
    NearestNeighbor,
    Linear,
    Quadratic,
}

/// Used to select how volume is scaled when mixing samples
/// 
/// - Off: no volume scaling, total volume will be much higher than individual
///        inputs
/// - Perceptual: maintains the perceived volume (RMS volume) roughly constant.
///               by dividing by the square root of the number of inputs
/// - Unity: maintains the total peak volume roughly constant. Arithmetic mean.
pub enum ScaleMethod {
    Off,
    Perceptual,
    Unity,
}

/// Used to select sample polarization
/// 
/// - Unity: no polarization (total polarization is +1)
/// - Zero:  total polarization is zero (phase cancellation)
/// - NegativeUnity: total polarization is -1
pub enum Polarization {
    Unity,
    Zero,
    NegativeUnity,
}