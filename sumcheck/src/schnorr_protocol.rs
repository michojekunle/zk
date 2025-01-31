use ark_ff::{UniformRand, Field};
use ark_std::rand::{rngs::OsRng, RngCore};
use crate::fiat_shamir::FiatShamir;
use sha2::{Digest, Sha256};
use ark_bn254::{Fq, Fr};

// Define a Schnorr struct
pub struct Schnorr {
    g: Fr, // Generator
    x: Fr, // Private key (secret)
    X: Fr, // Public key (X = g^x)
}

impl Schnorr {
    // Key Generation
    pub fn new(generator: Fr) -> Self {
        let mut rng = OsRng;
        let x = Fr::rand(&mut rng); // Secret key
        let X = generator.pow(x.into_repr()); // Public key
        Schnorr { g: generator, x, X }
    }

    // Non-Interactive Schnorr Signature using Fiat-Shamir
    pub fn sign(&self, message: &[u8]) -> (Fr, Fr) {
        let mut rng = OsRng;
        let r = Fr::rand(&mut rng); // Random nonce
        let R = self.g.pow(r.into_repr()); // Commitment

        // Use Fiat-Shamir to generate c = H(R, X, message)
        let mut transcript = FiatShamir::<Sha256, Fr>::new();
        transcript.absorb(&[R, self.X]); // Absorb commitment and public key
        transcript.absorb(message); // Absorb message
        let c: Fr = transcript.squeeze(); // Get challenge

        let s = r + c * self.x; // Response
        (R, s) // Return signature (R, s)
    }

    // Verification of Non-Interactive Schnorr
    pub fn verify(&self, message: &[u8], R: Fr, s: Fr) -> bool {
        // Recompute c using Fiat-Shamir
        let mut transcript = FiatShamir::<Sha256, Fr>::new();
        transcript.absorb(&[R, self.X]);
        transcript.absorb_bytes(message);
        let c: Fr = transcript.squeeze();

        // Check if g^s == R * X^c
        let lhs = self.g.pow(s.into_repr());
        let rhs = R * self.X.pow(c.into_repr());

        lhs == rhs
    }
}

fn main() {
    let generator = Fr::from(5u64); // Example generator

    let schnorr = Schnorr::new(generator);
    let message = b"Hello, Schnorr!";
    
    let (R, s) = schnorr.sign(message);
    let is_valid = schnorr.verify(message, R, s);

    println!("Signature valid: {}", is_valid);
}
