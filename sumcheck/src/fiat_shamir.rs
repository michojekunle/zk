// implementing fiat-shamir heuristic for removing interactivity in sumcheck protocol
use ark_ff::PrimeField;
use ark_std::rand::rngs::StdRng;
use ark_std::rand::SeedableRng;
use digest::{Digest, FixedOutputReset};
use std::marker::PhantomData;

pub(crate) struct FiatShamir<T: Digest, F: PrimeField> {
    pub(crate) hasher: T,
    pub(crate) _field: PhantomData<F>,
}

impl<T: Digest + Default + FixedOutputReset, F: PrimeField> FiatShamir<T, F> {
    pub(crate) fn new() -> Self {
        Self {
            hasher: T::new(),
            _field: PhantomData,
        }
    }

    pub(crate) fn absorb(&mut self, data: &[u8]) {
        for elem in data {
            let bytes = elem.to_le_bytes();
            Digest::update(&mut self.hasher, &bytes);
        }
    }

    pub(crate) fn squeeze(&mut self) -> F {
        let hash_result = self.hasher.finalize_reset();
        let seed: [u8; 32] = hash_result.as_slice()[..32].try_into().unwrap(); // Ensure correct size (32 bytes)
        let mut rng = StdRng::from_seed(seed);
        F::rand(&mut rng)
    }

    pub(crate) fn reset(&mut self) {
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

        let element = 42;

        transcript.absorb(&[element]);
        let random_element = transcript.squeeze();

        assert_ne!(random_element, Fq::from(element)); // verify randomness
    }
}
