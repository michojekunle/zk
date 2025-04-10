use ark_ec::pairing::Pairing;
use ark_ec::PrimeGroup;
use ark_ff::{PrimeField, UniformRand};
use ark_std::rand::rngs::StdRng;
use ark_std::rand::SeedableRng;
use std::ops::Mul;

/// Generates an array of Lagrange basis polynomials evaluated over the boolean hypercube
/// for a given set of `taus`.
pub fn generate_lagrange_basis<F: PrimeField>(taus: &[F]) -> Vec<F> {
    let n = taus.len();
    let mut lagrange_basis = Vec::new();

    let dim = 1 << n; // 2^n for the boolean hypercube
    for i in 0..dim {
        let mut product = F::one();
        for j in 0..n {
            let bit = (i >> j) & 1;
            if bit == 1 {
                product *= taus[j];
            } else {
                product *= F::one() - taus[j];
            }
        }
        lagrange_basis.push(product);
    }

    lagrange_basis
}

/// Encrypts the generated Lagrange basis polynomials using generator points G1.
pub fn encrypt_lagrange_basis<E: Pairing, F: PrimeField>(lagrange_basis: &[F]) -> Vec<E::G1> {
    lagrange_basis
        .iter()
        .map(|coeff| E::G1::generator().mul_bigint(coeff.into_bigint()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_bn254::Fr;
    use ark_bls12_381::Bls12_381;

    #[test]
    fn test_generate_lagrange_basis() {
        // Generate random taus for testing
        let mut rng = StdRng::from_entropy();
        let taus: Vec<Fr> = (0..3).map(|_| Fr::rand(&mut rng)).collect();

        let lagrange_basis = generate_lagrange_basis(&taus);

        // Basic assertions to ensure the function works as expected
        assert_eq!(lagrange_basis.len(), 1 << taus.len());
    }

    #[test]
    fn test_encrypt_lagrange_basis() {
        let mut rng = StdRng::from_entropy();
        let taus: Vec<Fr> = (0..3).map(|_| Fr::rand(&mut rng)).collect();
        let lagrange_basis = generate_lagrange_basis(&taus);

        // Encrypt the generated Lagrange basis polynomials
        let encrypted_basis = encrypt_lagrange_basis::<Bls12_381, Fr>(&lagrange_basis);

        // Basic assertions to ensure the function works as expected
        assert_eq!(encrypted_basis.len(), lagrange_basis.len());
    }
}
