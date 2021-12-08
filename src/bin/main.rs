/*
use dsp_lab::utils::math::par_shaper;
//use dsp_lab::core::dft::{SlidingDft, inverse_dft};
use dsp_lab::core::chaos::NoiseWhite;
use dsp_lab::traits::Source;
use fastapprox::{fast, faster};
use criterion::black_box;

 */

/*
fn main() {
    /*
    println!("TESTING SIN APPROXIMATION");
    let test_vals: [f64; 64] = [0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0, 1.1, 1.2, 1.3, 1.4, 1.5, 1.6, 1.7, 1.8, 1.9, 2.0, 2.1, 2.2, 2.3, 2.4, 2.5, 2.6, 2.7, 2.8, 2.9, 3.0, 3.1, 3.2, 3.3, 3.4, 3.5, 3.6, 3.7, 3.8, 3.9, 4.0, 4.1, 4.2, 4.3, 4.4, 4.5, 4.6, 4.7, 4.8, 4.9, 5.0, 5.1, 5.2, 5.3, 5.4, 5.5, 5.6, 5.7, 5.8, 5.9, 6.0, 6.1, 6.2, 6.3];
    let mut min = 99.0;
    let mut max = -1.0;
    let mut avg = 0.0;
    for val in test_vals{
        let std = val.sin();
        let fast = par_shaper(val);
        let err = (std - fast).abs();
        if err > max {
            max = err;
        }
        if err < min {
            min = err;
        }
        avg += err;
    }
    avg /= 64.0;
    println!("max error: {}", max);
    println!("min error: {}", min);
    println!("avg error: {}", avg);
    println!("");

    println!("TESTING FASTAPPROX FAST SIN");
    let test_vals: [f64; 64] = [0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0, 1.1, 1.2, 1.3, 1.4, 1.5, 1.6, 1.7, 1.8, 1.9, 2.0, 2.1, 2.2, 2.3, 2.4, 2.5, 2.6, 2.7, 2.8, 2.9, 3.0, 3.1, 3.2, 3.3, 3.4, 3.5, 3.6, 3.7, 3.8, 3.9, 4.0, 4.1, 4.2, 4.3, 4.4, 4.5, 4.6, 4.7, 4.8, 4.9, 5.0, 5.1, 5.2, 5.3, 5.4, 5.5, 5.6, 5.7, 5.8, 5.9, 6.0, 6.1, 6.2, 6.3];
    let mut min = 99.0;
    let mut max = -1.0;
    let mut avg = 0.0;
    for val in test_vals{
        let std = val.sin();
        let fast = fast::sinfull(val as f32) as f64;
        let err = (std - fast).abs();
        if err > max {
            max = err;
        }
        if err < min {
            min = err;
        }
        avg += err;
    }
    avg /= 64.0;
    println!("max error: {}", max);
    println!("min error: {}", min);
    println!("avg error: {}", avg);
    println!("");

    println!("TESTING FASTAPPROX FASTER SIN");
    let test_vals: [f64; 64] = [0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0, 1.1, 1.2, 1.3, 1.4, 1.5, 1.6, 1.7, 1.8, 1.9, 2.0, 2.1, 2.2, 2.3, 2.4, 2.5, 2.6, 2.7, 2.8, 2.9, 3.0, 3.1, 3.2, 3.3, 3.4, 3.5, 3.6, 3.7, 3.8, 3.9, 4.0, 4.1, 4.2, 4.3, 4.4, 4.5, 4.6, 4.7, 4.8, 4.9, 5.0, 5.1, 5.2, 5.3, 5.4, 5.5, 5.6, 5.7, 5.8, 5.9, 6.0, 6.1, 6.2, 6.3];
    let mut min = 99.0;
    let mut max = -1.0;
    let mut avg = 0.0;
    for val in test_vals{
        let std = val.sin();
        let fast = faster::sinfull(val as f32) as f64;
        let err = (std - fast).abs();
        if err > max {
            max = err;
        }
        if err < min {
            min = err;
        }
        avg += err;
    }
    avg /= 64.0;
    println!("max error: {}", max);
    println!("min error: {}", min);
    println!("avg error: {}", avg);
    println!("");
    */
    let mut dft = SlidingDft::new();
    let mut noise = NoiseWhite::new(1);
    loop {
        let frame = black_box(dft.step(noise.step()));
        black_box(inverse_dft(frame));
    }
}

 */
fn main() {}
