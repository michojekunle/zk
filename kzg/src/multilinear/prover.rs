use crate::multilinear::protocol::MultilinearKZGProof;
use crate::multilinear::utils::{encrypt_lagrange_basis, generate_lagrange_basis};
use ark_bls12_381::{Bls12_381, G1Projective};
use ark_bn254::{Bn254, Fr};
use ark_ec::pairing::Pairing;
use ark_ec::PrimeGroup;
use ark_ff::{PrimeField, UniformRand, Zero};
use ark_std::rand::rngs::StdRng;
use ark_std::rand::SeedableRng;
use polynomials::multilinear::multilinear_poly::{MultilinearPoly, BlowUpDirection};
use std::cmp::max;
use std::marker::PhantomData;
use std::ops::Mul;

#[derive(Clone, Debug)]
pub struct MultilinearKZGProver<F: PrimeField, E: Pairing> {
    _field: PhantomData<F>,
    _pairing: PhantomData<E>,
}

impl<F: PrimeField, E: Pairing> MultilinearKZGProver<F, E> {
    fn evaluate_at_tau(poly: &MultilinearPoly<F>, encrypted_lagrange_basis: &[E::G1]) -> E::G1 {
        dbg!(&poly.evals.len());
        dbg!(&encrypted_lagrange_basis.len());
        assert!(
            poly.evals.len() == encrypted_lagrange_basis.len(),
            "Length mismatch"
        );

        let mut result = E::G1::zero();
        for (i, eval) in poly.evals.iter().enumerate() {
            result += &encrypted_lagrange_basis[i].mul_bigint(&eval.into_bigint());
        }

        result
    }

    /// Computes the KZG commitment for a given polynomial.
    pub fn compute_commitment(
        poly: &MultilinearPoly<F>,
        encrypted_lagrange_basis: &[E::G1],
    ) -> E::G1 {
        Self::evaluate_at_tau(poly, encrypted_lagrange_basis)
    }

    /// Computes the KZG proof for a given polynomial and opening points.
    pub fn prove(
        openings: &[F],
        poly: &MultilinearPoly<F>,
        encrypted_lagrange_basis: &[E::G1],
    ) -> MultilinearKZGProof<F, E> {
        let v: F = poly.evaluate(openings.to_vec());
        let mut q_taus = Vec::with_capacity(openings.len());

        let f_minus_v = MultilinearPoly::new(
            poly.evals.iter().map(|eval| *eval - v).collect(),
            poly.n_vars,
        );

        let mut dividend = f_minus_v;

        for (i, opening) in openings.iter().enumerate() {
            // divide the polynomial by each opening as a factor
            // e.g. if the roots are a = 6, b = 7, c = 0; we divide the polynomial by a - 6, remainder by b - 7 and lastly, c - 0;
            // But in actual fact, we are evaluating the polynomial at the variable points.
            let (mut quotient, remainder) = dividend.compute_quotient_remainder(opening, dividend.n_vars - 1);

            dividend = remainder;

            quotient = MultilinearPoly::blow_up_n_times(
                BlowUpDirection::Left,
                &quotient,
                max(i + 1, openings.len() - quotient.len().ilog2() as usize),
            );

            let q_tau: E::G1 = Self::evaluate_at_tau(
                &MultilinearPoly::new(quotient.to_vec(), quotient.len().ilog2() as usize),
                encrypted_lagrange_basis,
            );

            q_taus.push(q_tau);
        }

        MultilinearKZGProof::new(v, q_taus)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::multilinear::trusted_setup::TrustedSetup;

    #[test]
    fn test_evaluate_at_tau() {
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

        let taus: Vec<Fr> = (0..3).map(|_| Fr::rand(&mut rng)).collect();
        let trusted_setup = TrustedSetup::<Bls12_381, Fr>::new(taus.as_slice());
        let encrypted_lagrange_basis = trusted_setup.encrypted_lagrange_basis;

        let result = MultilinearKZGProver::<Fr, Bls12_381>::evaluate_at_tau(
            &poly,
            &encrypted_lagrange_basis,
        );

        assert!(!result.is_zero(), "Result should not be zero");
    }

    #[test]
    fn test_compute_commitment() {
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

        let taus: Vec<Fr> = (0..3).map(|_| Fr::rand(&mut rng)).collect();
        let trusted_setup = TrustedSetup::<Bls12_381, Fr>::new(taus.as_slice());
        let encrypted_lagrange_basis = trusted_setup.encrypted_lagrange_basis;

        let commitment = MultilinearKZGProver::<Fr, Bls12_381>::compute_commitment(
            &poly,
            &encrypted_lagrange_basis,
        );

        assert!(!commitment.is_zero(), "Commitment should not be zero");
    }

    #[test]
    fn test_prove() {
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

        let taus: Vec<Fr> = (0..3).map(|_| Fr::rand(&mut rng)).collect();
        let trusted_setup = TrustedSetup::<Bls12_381, Fr>::new(taus.as_slice());
        let encrypted_lagrange_basis = trusted_setup.encrypted_lagrange_basis;
        let openings: Vec<Fr> = (0..3).map(|_| Fr::rand(&mut rng)).collect();

        let proof = MultilinearKZGProver::<Fr, Bls12_381>::prove(
            &openings,
            &poly,
            &encrypted_lagrange_basis,
        );

        assert_eq!(
            proof.q_taus.len(),
            3,
            "Proof should contain q_taus for each variable"
        );
        assert!(!proof.v.is_zero(), "Proof value should not be zero");
    }
}
