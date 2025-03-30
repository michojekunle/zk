use ark_ec::pairing::Pairing;
use ark_ff::{BigInteger, PrimeField};
use polynomials::multilinear::multilinear_poly::MultilinearPoly;
use sha3::Keccak256;
use std::marker::PhantomData;
use sumcheck::fiat_shamir::FiatShamir;
use sumcheck::sumcheck_protocol::PartialProof;
use crate::prover::GKRProver;
use crate::verifier::GKRVerifier;
use crate::circuit::{Circuit, Op, Gate};

pub struct GKRProof<F: PrimeField> {
    pub output_poly: MultilinearPoly<F>,
    pub w_poly_evals: Vec<(F, F)>,
    pub sumcheck_proofs: Vec<PartialProof<F>>,
}

pub struct GKRProofWithKZG<F: PrimeField, P: Pairing> {
    _marker: PhantomData<F>,
    _marker_p: PhantomData<P>,
}

impl<F: PrimeField> GKRProof<F> {
    pub fn new(
        output_poly: MultilinearPoly<F>,
        w_poly_evals: Vec<(F, F)>,
        sumcheck_proofs: Vec<PartialProof<F>>,
    ) -> Self {
        Self {
            output_poly,
            w_poly_evals,
            sumcheck_proofs,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_bn254::Fq;

    fn init_circuit() -> (Circuit<Fq>, Vec<Fq>) {
        let gate_a = Gate::new(0, 1, 0, Op::ADD);
        let gate_b = Gate::new(2, 3, 1, Op::MUL);
        let gate_c = Gate::new(4, 5, 2, Op::MUL);
        let gate_d = Gate::new(6, 7, 3, Op::MUL);
        let gate_e = Gate::new(0, 1, 0, Op::ADD);
        let gate_f = Gate::new(2, 3, 1, Op::MUL);
        let gate_g = Gate::new(0, 1, 0, Op::ADD);

        let layer_2 = vec![gate_a, gate_b, gate_c, gate_d];
        let layer_1 = vec![gate_e, gate_f];
        let layer_0 = vec![gate_g];

        let circuit = Circuit::<Fq>::new(vec![layer_0, layer_1, layer_2]);

        let input = vec![
            Fq::from(1u64),
            Fq::from(2u64),
            Fq::from(3u64),
            Fq::from(4u64),
            Fq::from(5u64),
            Fq::from(6u64),
            Fq::from(7u64),
            Fq::from(8u64),
        ];

        (circuit, input)
    }

    #[test]
    fn test_gkr_impl() {
        let (mut circuit, input) = init_circuit();
        let mut transcript = FiatShamir::<Keccak256, Fq>::new();

        let gkr_proof = GKRProver::prove(&input, &mut circuit, &mut transcript);
    }
}
