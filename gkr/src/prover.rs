use crate::circuit::Circuit;
use crate::protocol::{GKRProof, GKRProofWithKZG};
use ark_ec::pairing::Pairing;
use ark_ff::{BigInteger, PrimeField};
use polynomials::{
    composed::{product_poly::ProductPoly, sum_poly::SumPoly},
    multilinear::multilinear_poly::MultilinearPoly,
};
use sha3::Keccak256;
use std::marker::PhantomData;
use sumcheck::{fiat_shamir::FiatShamir, sumcheck_protocol::partial_prove};

pub struct GKRProver<F: PrimeField, P: Pairing> {
    _marker: PhantomData<F>,
    _marker_p: PhantomwData<P>,
}

impl<F: PrimeField, P: Pairing> GKRProver<F, P> {
    pub fn prove(
        input_layer: &[F],
        circuit: &mut Circuit<F>,
        transcript: &mut FiatShamir<Keccak256, F>,
        proof: GKRProof<F>,
    ) -> GKRProof<F> {

        todo!();
    }
}
