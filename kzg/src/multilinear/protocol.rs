use crate::multilinear::{
    prover::MultilinearKZGProver, trusted_setup::TrustedSetup, verifier::MultilinearKZGVerifier,
};
use ark_bls12_381::{Fr, Bls12_381, G1Projective};
use ark_ec::{pairing::Pairing, PrimeGroup};
use ark_ff::{PrimeField, UniformRand, Zero};
use ark_std::rand::{rngs::StdRng, SeedableRng};
use polynomials::multilinear::multilinear_poly::{BlowUpDirection, MultilinearPoly};
use std::{cmp::max, marker::PhantomData, ops::Mul};

#[derive(Debug)]
pub struct MultilinearKZGProof<F: PrimeField, E: Pairing> {
    pub v: F,
    pub q_taus: Vec<E::G1>,
}

impl<F: PrimeField, E: Pairing> MultilinearKZGProof<F, E> {
    pub fn new(v: F, q_taus: Vec<E::G1>) -> Self {
        Self { v, q_taus }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kzg_protocol() {
        let mut rng = StdRng::from_entropy();
        let poly = MultilinearPoly::new(
            vec![
                Fr::from(0),
                Fr::from(0),
                Fr::from(0),
                Fr::from(3),
                Fr::from(0),
                Fr::from(0),
                Fr::from(2),
                Fr::from(5),
            ],
            3,
        );

        // let taus: Vec<Fr> = (0..3).map(|_| Fr::rand(&mut rng)).collect();
        let trusted_setup = TrustedSetup::<Bls12_381, Fr>::new(&[
            Fr::from(5),
            Fr::from(3),
            Fr::from(2),
        ]);
        // let trusted_setup = TrustedSetup::<Bls12_381, Fr>::new(taus.as_slice());

        // let openings: Vec<Fr> = (0..3).map(|_| Fr::rand(&mut rng)).collect();
        let openings: Vec<Fr> = vec![
            Fr::from(2),
            Fr::from(4),
            Fr::from(0),
        ];
        
        let commitment = MultilinearKZGProver::<Fr, Bls12_381>::compute_commitment(&poly, &trusted_setup.encrypted_lagrange_basis);

        let proof = MultilinearKZGProver::<Fr, Bls12_381>::prove(
            &openings,
            &poly,
            &trusted_setup.encrypted_lagrange_basis,
        );

        dbg!(&proof);

        let is_verified = MultilinearKZGVerifier::<Fr, Bls12_381>::verify(
            &commitment,
            &openings,
            &proof,
            &trusted_setup.encrypted_taus,
        );

        dbg!(&is_verified);
        assert!(is_verified, "Proof verification failed");
    }
}
