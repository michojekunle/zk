use sumcheck::fiat_shamir::FiatShamir;
use polynomials::multilinear::multilinear_poly::MultilinearPoly;
use ark_ff::{BigInteger, PrimeField};
use sha3::Keccak256;
use sumcheck::sumcheck_protocol::{partial_prove, partial_verify};

struct GKRProof {}

pub(crate) fn GKRProve() {
    todo!()
}

pub(crate) fn GKRVerify() {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gkr_implementation() {}
}
