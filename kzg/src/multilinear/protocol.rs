use std::cmp::max;
use polynomials::multilinear::multilinear_poly::MultilinearPoly;
use ark_ec::pairing::Pairing;
use ark_ec::PrimeGroup;
use ark_ff::{PrimeField, UniformRand};
use std::ops::Mul;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct MultilinearKZGProof<F: PrimeField, E: Pairing> {
    _marker: PhantomData<F>,
    pub v: F,
    pub q_taus: Vec<E::G1>,
}

impl<F: PrimeField, E: Pairing> MultilinearKZGProof<F, E> {
    pub fn new(v: F, q_taus: Vec<E::G1>) -> Self {
        Self {
            _marker: PhantomData,
            v,
            q_taus,
        }
    }
}