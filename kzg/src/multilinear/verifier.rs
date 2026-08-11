use crate::multilinear::{
    protocol::MultilinearKZGProof, prover::MultilinearKZGProver, trusted_setup::TrustedSetup,
};
use ark_bls12_381::{Bls12_381, G1Projective};
use ark_ec::{pairing::Pairing, PrimeGroup};
use ark_ff::{PrimeField, UniformRand, Zero};
use ark_std::rand::{rngs::StdRng, SeedableRng};
use polynomials::multilinear::multilinear_poly::{BlowUpDirection, MultilinearPoly};
use std::{cmp::max, marker::PhantomData, ops::Mul};

#[derive(Clone, Debug)]
pub struct MultilinearKZGVerifier<F: PrimeField, E: Pairing> {
    _field: PhantomData<F>,
    _pairing: PhantomData<E>,
}

impl<F: PrimeField, E: Pairing> MultilinearKZGVerifier<F, E> {
    pub fn verify(
        commitment: &E::G1,
        openings: &[F],
        proof: &MultilinearKZGProof<F, E>,
        encrypted_taus: &[E::G2],
    ) -> bool {
        let g1_v = E::G1::generator().mul_bigint(proof.v.into_bigint());
        let f_tau_minus_v = *commitment - g1_v;
        let g2_1 = E::G2::generator().mul_bigint(F::one().into_bigint());

        let lhs = E::pairing(f_tau_minus_v, g2_1);

        let rhs_calc = proof
            .q_taus
            .iter()
            .enumerate()
            .map(|(i, q_tau)| {
                let tau_i = encrypted_taus[i];
                let g2_a = E::G2::generator().mul_bigint(openings[i].into_bigint());

                E::pairing(*q_tau, tau_i - g2_a)
            })
            .collect::<Vec<_>>();

        let rhs = rhs_calc.iter().sum();

        // dbg!(&rhs);
        // dbg!(&lhs);

        lhs == rhs
    }
}
