use std::cmp::max;
use polynomials::multilinear::multilinear_poly::MultilinearPoly;
use ark_ec::pairing::Pairing;
use ark_ec::PrimeGroup;
use ark_ff::{PrimeField, UniformRand, Zero};
use std::ops::Mul;
use std::marker::PhantomData;
use crate::multilinear::protocol::MultilinearKZGProof;

#[derive(Clone, Debug)]
pub struct MultilinearKZGProver<F: PrimeField, E: Pairing,> {
    _field: PhantomData<F>,
    _pairing: PhantomData<E>,
}

impl < F: PrimeField, E: Pairing> MultilinearKZGProver<F, E> {
    fn evaluate_at_tau(
        poly: &MultilinearPoly<F>,
        encrypted_lagrange_basis: &[E::G1],
    ) -> E::G1 {
        assert!(poly.evals.len() == encrypted_lagrange_basis.len(), "Length mismatch");

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

    /// Computes the KZG proof for a given polynomial and point.
    pub fn prove(
        openings: &[F],
        poly: &MultilinearPoly<F>,
        encrypted_lagrange_basis: &[E::G1],
    ) -> MultilinearKZGProof<F, E> {
        let v = poly.evaluate(openings.to_vec());
        // let mut proof = E::G1::zero();
        // for (i, coeff) in poly.coefficients.iter().enumerate() {
        //     let point = setup.encrypted_lagrange_basis[i];
        //     proof.add_assign(&point.mul(coeff));
        // }
        // proof.mul(point)
        // let mut q_taus = Vec::new();
        todo!();
    }
}



