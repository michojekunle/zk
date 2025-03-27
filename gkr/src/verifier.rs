use crate::circuit::Circuit;
use ark_ff::{BigInteger, PrimeField};
use ark_ec::pairing::Pairing;
use std::marker::PhantomData;
use polynomials::multilinear::multilinear_poly::MultilinearPoly;
use sha3::Keccak256;
use sumcheck::{fiat_shamir::FiatShamir, sumcheck_protocol::partial_verify};
use crate::protocol::{GKRProof, GKRProofWithKZG};

pub struct GKRVerifier<F: PrimeField, P: Pairing> {
    _phantom: PhantomData<F>,
    _phantom_p: PhantomData<P>
}

impl <F: PrimeField, P: Pairing> GKRVerifier<F, P> {
    pub fn verify(
        input_layer: &[F],
        circuit: &mut Circuit<F>,
        transcript: &mut FiatShamir<Keccak256, F>,
        proof: GKRProof<F>,
    ) -> bool {
        
        true
    }
}