use crate::fiat_shamir::FiatShamir;
use crate::multilinear_poly::MultilinearPoly;
use ark_ff::{BigInteger, PrimeField};
use sha3::Keccak256;
use sumcheck::{PartialProve, PartialVerify};

struct GKRProof {}

pub(crate) fn GKRProve() {
    todo!()
}

pub(crate) fn GKRVerify() {
    todo!()
}

#[cfg(tests)]
mod tests {
    use super::*;

    #[test]
    fn test_gkr_implementation() {}
}
