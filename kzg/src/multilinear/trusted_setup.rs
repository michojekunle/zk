use crate::multilinear::utils::{generate_lagrange_basis, encrypt_lagrange_basis};
use ark_ec::pairing::Pairing;
use ark_ec::PrimeGroup;
use ark_ff::PrimeField;
use std::ops::Mul;
use ark_std::UniformRand;
use std::marker::PhantomData;

pub struct TrustedSetup<E: Pairing, F: PrimeField> {
    pub encrypted_taus: Vec<E::G2>,
    pub encrypted_lagrange_basis: Vec<E::G1>,
    _marker: PhantomData<F>,   
}

impl<E: Pairing, F: PrimeField> TrustedSetup<E, F> {
    pub fn new(taus: &[F]) -> Self {
        // Generate random taus for testing
        let encrypted_taus: Vec<E::G2> = taus
            .iter()
            .map(|tau| E::G2::generator().mul_bigint(tau.into_bigint()))
            .collect();

        // Generate Lagrange basis polynomials evaluated over the boolean hypercube
        // for the given set of `taus`.
        let lagrange_basis = generate_lagrange_basis(taus);
        let encrypted_lagrange_basis = encrypt_lagrange_basis::<E, F>(&lagrange_basis);

        Self {
            _marker: PhantomData,
            encrypted_taus,
            encrypted_lagrange_basis,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_bn254::Fr;
    use ark_std::rand::rngs::StdRng;
    use ark_bls12_381::Bls12_381;
    use ark_std::rand::SeedableRng;

    #[test]
    fn test_trusted_setup() {
        let mut rng = StdRng::from_entropy();
        let taus: Vec<Fr> = (0..3).map(|_| Fr::rand(&mut rng)).collect();

        let setup = TrustedSetup::<Bls12_381, Fr>::new(&taus);

        // Print the results for verification
        // println!("Encrypted Taus: {:?}", setup.encrypted_taus);
        // println!("Encrypted Lagrange Basis: {:?}", setup.encrypted_lagrange_basis);
        // You can add assertions here to check the correctness of the setup
        // For example, check the length of the encrypted taus and basis
        assert_eq!(setup.encrypted_taus.len(), taus.len());
        assert_eq!(setup.encrypted_lagrange_basis.len(), 1 << taus.len());
        // Check if the encrypted taus are not empty
        assert!(!setup.encrypted_taus.is_empty());
        // Check if the encrypted lagrange basis is not empty
        assert!(!setup.encrypted_lagrange_basis.is_empty());
        // Check if the encrypted lagrange basis is the expected size
        assert_eq!(setup.encrypted_lagrange_basis.len(), 1 << taus.len());
    }
}

