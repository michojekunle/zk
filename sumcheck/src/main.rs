use sumcheck::fiat_shamir::FiatShamir;
// use sumcheck::schnorr_protocol::Schnorr;
use sumcheck::sumcheck_protocol::Sumcheck;
use sha2::Sha256;
use ark_bn254::Fq;
use ark_ff::PrimeField;

fn main() {    
    let mut transcript = FiatShamir::<Sha256, Fq>::new();

    // Convert byte slice to field element
    let commitment_1 = Fq::from_le_bytes_mod_order(b"commitment_1");
    let commitment_2 = Fq::from_le_bytes_mod_order(b"commitment_2");

    // Absorb field elements
    transcript.absorb(&[commitment_1]);
    transcript.absorb(&[commitment_2]);

    let challenge = transcript.squeeze();

    println!("Fiat-Shamir challenge: {:?}", challenge);
}
