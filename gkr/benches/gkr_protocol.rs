use ark_bn254::Fq;
use criterion::{criterion_group, criterion_main, Criterion};
use gkr::circuit::{Circuit, Gate, Op};
use gkr::prover::GKRProver;
use gkr::verifier::GKRVerifier;
use sha3::Keccak256;
use sumcheck::fiat_shamir::FiatShamir;
use std::hint::black_box;

fn build_circuit(input_len: usize) -> (Circuit<Fq>, Vec<Fq>) {
    let mut layers = Vec::new();

    // Layer closest to the inputs.
    let mut input_layer = Vec::new();

    for i in (0..input_len).step_by(2) {
        input_layer.push(Gate::new(
            i,
            i + 1,
            i / 2,
            if i % 4 == 0 { Op::ADD } else { Op::MUL },
        ));
    }

    layers.push(input_layer);

    // Keep reducing the number of values by half
    // until we reach one output.
    let mut current_size = input_len / 2;

    while current_size > 1 {
        let mut layer = Vec::new();

        for i in (0..current_size).step_by(2) {
            layer.push(Gate::new(
                i,
                i + 1,
                i / 2,
                if i % 4 == 0 { Op::ADD } else { Op::MUL },
            ));
        }

        layers.push(layer);
        current_size /= 2;
    }

    // GKR expects layers in output -> input order.
    layers.reverse();

    let input = (1..=input_len).map(|i| Fq::from(i as u64)).collect();

    (Circuit::new(layers, input_len), input)
}

fn benchmark_gkr(c: &mut Criterion, input_size: usize) {
    let (mut circuit, input) = build_circuit(input_size);
    let mut transcript_p = FiatShamir::<Keccak256, Fq>::new();
    let mut transcript_v = FiatShamir::<Keccak256, Fq>::new();

    let gkr_proof = GKRProver::prove(&input, &mut circuit, &mut transcript_p);

    // bench prover
    c.bench_function(&format!("gkr_prover/{}_inputs", input_size), |b| {
        b.iter(|| black_box(GKRProver::prove(&input, &mut circuit, &mut transcript_p)))
    });

    // bench verifier
    c.bench_function(&format!("gkr_verifier/{}_inputs", input_size), |b| {
        b.iter(|| black_box(GKRVerifier::verify(&input, &mut circuit, &mut transcript_v, &gkr_proof)))
    });
}

fn gkr_benchmarks(c: &mut Criterion) {
    for size in [8, 64, 256] {
        benchmark_gkr(c, size);
    }
}

criterion_group!(benches, gkr_benchmarks);
criterion_main!(benches);
