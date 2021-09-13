use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use dsp_lab::utils::math::{fast_sin, fast_round, par_shaper};
use dsp_lab::core::dft::{SlidingDft, inverse_dft};
use dsp_lab::core::chaos::NoiseWhite;
use dsp_lab::traits::Source;
use fastapprox::{fast, faster};


pub fn criterion_benchmark(c: &mut Criterion) {
    /*
    let mut group_1 = c.benchmark_group("round approx");
    for x in [0.0, 0.25, 0.75, 1.0, 1.25, 1.75, -0.25, -0.75, -1.25, -1.75].iter() {
        group_1.bench_with_input(BenchmarkId::new("approx", x), x, 
            |b, x| b.iter(|| fast_round(*x)));
        group_1.bench_with_input(BenchmarkId::new("std", x), x, 
            |b, x| b.iter(|| (*x).round()));
    }
    group_1.finish();
    */

    /*
    let mut group_2 = c.benchmark_group("sin approx");
    for x in [-100.0, -10.0, -1.0, -0.1, 0.0, 0.1, 10.0, 100.0].iter() {
        group_2.bench_with_input(BenchmarkId::new("par_shaper", x), x, 
            |b, x| b.iter(|| par_shaper(*x)));
        group_2.bench_with_input(BenchmarkId::new("std", x), x, 
            |b, x| b.iter(|| (*x).sin()));
        group_2.bench_with_input(BenchmarkId::new("fastapprox fast", x), x, 
            |b, x| b.iter(|| fast::sinfull(*x as f32)));
        group_2.bench_with_input(BenchmarkId::new("fastapprox faster", x), x, 
            |b, x| b.iter(|| faster::sinfull(*x as f32)));
    }
    group_2.finish();
    */
    


    c.bench_function("dft", |b| b.iter(|| {
        let mut sdft = SlidingDft::new();
        let mut noise = NoiseWhite::new(1);
        for _ in 0..256 {       // test a full frame of dft at a time
            black_box(inverse_dft(sdft.step(black_box(noise.step()))));
        }
    }));

}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);