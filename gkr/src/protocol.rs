use ark_ff::{BigInteger, PrimeField};
use polynomials::multilinear::multilinear_poly::MultilinearPoly;
use sha3::Keccak256;
use sumcheck::fiat_shamir::FiatShamir;
use sumcheck::sumcheck_protocol::{partial_prove, partial_verify, PartialProof};
use ark_ec::pairing::Pairing;
use std::marker::PhantomData;

pub struct GKRProof<F: PrimeField> {
    pub output_poly: MultilinearPoly<F>,
    pub w_poly_evals: Vec<(F, F)>,
    pub sumcheck_proofs: Vec<PartialProof<F>>,
}

pub struct GKRProofWithKZG<F: PrimeField, P: Pairing> {
    _marker: PhantomData<F>,
    _marker_p: PhantomData<P>,
}

impl<F: PrimeField> GKRProof<F> {
    pub fn new(
        output_poly: MultilinearPoly<F>,
        w_poly_evals: Vec<(F, F)>,
        sumcheck_proofs: Vec<PartialProof<F>>,
    ) -> Self {
        Self {
            output_poly,
            w_poly_evals,
            sumcheck_proofs,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gkr_sumcheck_impl() {}
}
