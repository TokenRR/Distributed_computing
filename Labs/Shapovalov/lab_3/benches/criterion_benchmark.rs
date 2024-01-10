use std::{sync::Arc};
use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use lab_3::integration::integral_reduction;


fn bench_integral_reduction(c: &mut Criterion) {
    let f = |t: f64| (t + 1.0).ln();
    let a = 0.0;
    let b = 1.0;

    let mut group = c.benchmark_group("Multithread integration ");

    for isteps in [10, 1000 as i32, 1e6 as i32] {
        for threads in [1,2,4,8] {
            let pool = rayon::ThreadPoolBuilder::new().num_threads(threads).build().unwrap();
            let af = Arc::new(f);
            let signature = format!(" f= ln(t + 1), a={}, b={}, steps={}, threads={}", a, b, isteps, threads);
            let args = (&af, a, b, isteps);
            group.bench_with_input(
                BenchmarkId::from_parameter(signature),
                &args,
                |b, inp| {
                    b.iter(|| pool.install(|| integral_reduction(inp.0,inp.1,inp.2,inp.3)));
                }
            );
        };
    };
    group.finish();
}


criterion_group!(benches, bench_integral_reduction);
criterion_main!(benches);
