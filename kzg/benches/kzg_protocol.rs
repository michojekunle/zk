use ark_bls12_381::{Bls12_381, Fr};
use ark_ff::UniformRand;
use ark_std::rand::{rngs::StdRng, SeedableRng};
use criterion::{criterion_group, criterion_main, Criterion};
use kzg::multilinear::{
    prover::MultilinearKZGProver, trusted_setup::TrustedSetup, verifier::MultilinearKZGVerifier,
};
use polynomials::multilinear::multilinear_poly::MultilinearPoly;

fn setup(n_vars: usize) -> (TrustedSetup<Bls12_381, Fr>, Vec<Fr>, MultilinearPoly<Fr>) {
    let mut rng = StdRng::seed_from_u64(42);

    // A multilinear polynomial over n variables has 2^n evaluations.
    let n_evals = 1usize << n_vars;

    let poly_values: Vec<Fr> = (0..n_evals).map(|i| Fr::from((i + 1) as u64)).collect();

    // One tau per variable.
    let taus: Vec<Fr> = (0..n_vars).map(|_| Fr::rand(&mut rng)).collect();

    // One opening coordinate per variable.
    let openings: Vec<Fr> = (0..n_vars).map(|_| Fr::rand(&mut rng)).collect();

    let poly = MultilinearPoly::new(poly_values, n_vars);

    let trusted_setup = TrustedSetup::<Bls12_381, Fr>::new(&taus);

    (trusted_setup, openings, poly)
}

fn kzg_benchmark(c: &mut Criterion, n_vars: usize) {
    let (trusted_setup, openings, poly) = setup(n_vars);

    c.bench_function(&format!("kzg_commit/{} variables", n_vars), |b| {
        b.iter(|| {
            criterion::black_box(MultilinearKZGProver::<Fr, Bls12_381>::compute_commitment(
                &poly,
                &trusted_setup.encrypted_lagrange_basis,
            ))
        })
    });

    c.bench_function(&format!("kzg_prover/{} variables", n_vars), |b| {
        b.iter(|| {
            criterion::black_box(MultilinearKZGProver::<Fr, Bls12_381>::prove(
                &openings,
                &poly,
                &trusted_setup.encrypted_lagrange_basis,
            ))
        })
    });

    let proof = MultilinearKZGProver::<Fr, Bls12_381>::prove(
        &openings,
        &poly,
        &trusted_setup.encrypted_lagrange_basis,
    );

    let commitment = MultilinearKZGProver::<Fr, Bls12_381>::compute_commitment(
        &poly,
        &trusted_setup.encrypted_lagrange_basis,
    );

    let valid = MultilinearKZGVerifier::<Fr, Bls12_381>::verify(
        &commitment,
        &openings,
        &proof,
        &trusted_setup.encrypted_taus,
    );

    assert!(valid);

    c.bench_function(&format!("kzg_verifier/{} variables", n_vars), |b| {
        b.iter(|| {
            criterion::black_box(MultilinearKZGVerifier::<Fr, Bls12_381>::verify(
                &commitment,
                &openings,
                &proof,
                &trusted_setup.encrypted_taus,
            ));
        })
    });
}

fn kzg_benchmarks(c: &mut Criterion) {
    for n_vars in [3, 4, 5, 6, 7, 8] {
        kzg_benchmark(c, n_vars);
    }
}

criterion_group!(benches, kzg_benchmarks);
criterion_main!(benches);
