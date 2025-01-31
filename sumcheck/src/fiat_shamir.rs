// implementing fiat-shamir heuristic for removing interactivity in sumcheck protocol
use ark_ff::{BigInteger, PrimeField};
use digest::{Digest, FixedOutputReset};
use std::marker::PhantomData;
use ark_std::rand::SeedableRng;
use ark_std::rand::rngs::StdRng;

pub struct FiatShamir<T: Digest, F: PrimeField> {
    hasher: T,
    _field: PhantomData<F>,
}

impl<T: Digest + Default + FixedOutputReset, F: PrimeField> FiatShamir<T, F> {
    pub fn new() -> Self {
        Self {
            hasher: T::new(),
            _field: PhantomData,
        }
    }

    pub fn absorb(&mut self, data: &[F]) {
        for field_elem in data {
            let bytes = field_elem.into_bigint().to_bytes_le(); // convert field elements to bytes
            Digest::update(&mut self.hasher, &bytes);
        }
    }

    pub fn squeeze(&mut self) -> F {
        let hash_result = self.hasher.finalize_reset();
        let seed: [u8; 32] = hash_result.as_slice()[..32].try_into().unwrap(); // Ensure correct size (32 bytes)
        let mut rng = StdRng::from_seed(seed);
        F::rand(&mut rng)
    }

    pub fn reset(&mut self) {
        self.hasher = T::new();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_bn254::Fq;
    use sha3::Keccak256;

    #[test]
    fn test_fiat_shamir_instantiation() {
        let _transcript = FiatShamir::<Keccak256, Fq>::new();
    }

    #[test]
    fn test_fiat_shamir_absorb_and_squeeze() {
        let mut transcript = FiatShamir::<Keccak256, Fq>::new();

        let field_element = Fq::from(42u64);

        transcript.absorb(&[field_element]);
        let random_field_element = transcript.squeeze();

        assert_ne!(random_field_element, field_element); // verify randomness
    }
}
