use ark_ff::{BigInteger, PrimeField};
use polynomials::multilinear::multilinear_poly::MultilinearPoly;
use sha3::Keccak256;
use sumcheck::fiat_shamir::FiatShamir;
use sumcheck::sumcheck_protocol::{partial_prove, partial_verify, PartialProof};

pub struct GKRProof<F: PrimeField> {
    pub output_poly: MultilinearPoly<F>,
    pub sumcheck_proofs: Vec<PartialProof<F>>,
    pub w_poly_evals: Vec<(F, F)>,
}

impl<F: PrimeField> GKRProof<F> {
    pub fn new(
        output_poly: MultilinearPoly<F>,
        sumcheck_proofs: Vec<PartialProof<F>>,
        w_poly_evals: Vec<(F, F)>,
    ) -> Self {
        Self {
            output_poly,
            sumcheck_proofs,
            w_poly_evals,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gkr_sumcheck_impl() {}
}
