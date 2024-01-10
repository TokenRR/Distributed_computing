use nalgebra::{DMatrix};
use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use lab_2::matop::{matmul, matmul_paralel, matelsum, matelsum_paralel};


fn bench_matmul(c: &mut Criterion) {
    let mut group = c.benchmark_group("Matmul");
    for (a, b) in [(150,150)] {
        let m1 = DMatrix::<f64>::new_random(a, b);
        let m2 = DMatrix::<f64>::new_random(b, a);
        group.bench_with_input(
            BenchmarkId::new("SingleThread", a),
            &(&m1, &m2),|b, inp| b.iter(|| matmul(inp.0, inp.1)) 
        );
        group.bench_with_input(
            BenchmarkId::new("Multithread", a),
            &(&m1, &m2),|b, inp| b.iter(|| matmul_paralel(inp.0, inp.1)) 
        );
    }
    group.finish();
}


fn bench_matelsum(c: &mut Criterion) {
    let mut group = c.benchmark_group("Matelsum");
    for (a, b) in [(1000,1000)] {
        let m = DMatrix::<f64>::new_random(a, b);
        group.bench_with_input(
            BenchmarkId::new("SingleThread", a),
            &m,|b, inp| b.iter(|| matelsum(inp)) 
        );
        group.bench_with_input(
            BenchmarkId::new("Multithread", a),
            &m,|b, inp| b.iter(|| matelsum_paralel(inp)) 
        );
    }
    group.finish();
}


criterion_group!(benches, bench_matmul, bench_matelsum);
criterion_main!(benches);
