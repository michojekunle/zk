// implementing fiat-shamir heuristic for removing interactivity in sumcheck protocol
use ark_ff::PrimeField;
use ark_std::rand::rngs::StdRng;
use ark_std::rand::SeedableRng;
use digest::{Digest, FixedOutputReset};
use std::marker::PhantomData;

pub struct FiatShamir<T: Digest, F: PrimeField> {
    pub hasher: T,
    pub _field: PhantomData<F>,
}

impl<T: Digest + Default + FixedOutputReset, F: PrimeField> FiatShamir<T, F> {
    pub fn new() -> Self {
        Self {
            hasher: T::new(),
            _field: PhantomData,
        }
    }

    pub fn absorb(&mut self, data: &[u8]) {
        for elem in data {
            let bytes = elem.to_le_bytes();
            Digest::update(&mut self.hasher, &bytes);
        }
    }

    pub fn absorb_n(&mut self, data: &[&[u8]]) {
        data.iter().for_each(|f| self.absorb(*f));
    }

    pub fn squeeze(&mut self) -> F {
        let hash_result = self.hasher.finalize_reset();
        let seed: [u8; 32] = hash_result.as_slice()[..32].try_into().unwrap(); // Ensure correct size (32 bytes)
        Digest::update(&mut self.hasher, &hash_result);
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

        let element = 42;

        transcript.absorb(&[element]);
        let random_element = transcript.squeeze();

        assert_ne!(random_element, Fq::from(element)); // verify randomness
    }

    #[test]
    fn test_sample_challenge_should_absorb_after_sampling() {
        let mut transcript = FiatShamir::<Keccak256, Fq>::new();

        let element = 42;

        transcript.absorb(&[element]);
        let random_element = transcript.squeeze();
        let random_element_i = transcript.squeeze();
        let random_element_j = transcript.squeeze();
        let random_element_k = transcript.squeeze();
        let random_element_l = transcript.squeeze();
        let random_element_m = transcript.squeeze();

        dbg!(&random_element);
        dbg!(&random_element_i);
        dbg!(&random_element_j);
        dbg!(&random_element_k);
        dbg!(&random_element_l);
        dbg!(&random_element_m);

        assert_ne!(random_element, Fq::from(element)); // verify randomness
    }
}
