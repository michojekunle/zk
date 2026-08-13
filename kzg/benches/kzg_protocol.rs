use criterion::{criterion_group, criterion_main, Criterion};

fn kzg_benchmark(c: &mut Criterion) {
    c.bench_function("kzg_commit", |b| b.iter(|| my_function()));
    
    c.bench_function("kzg_prover", |b| b.iter(|| my_function()));
    c.bench_function("kzg_verifier", |b| b.iter(|| my_function()));
}

criterion_group!(benches, kzg_benchmark);
criterion_main!(benches);
