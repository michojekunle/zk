use criterion::{criterion_group, criterion_main, Criterion};

fn my_benchmark(c: &mut Criterion) {
    c.bench_function("my_function", |b| b.iter(|| my_function()));
}

criterion_group!(benches, my_benchmark);
criterion_main!(benches);