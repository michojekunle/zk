use crate::circuit::{Circuit, Op};
use crate::{
    protocol::{GKRProof, GKRProofWithKZG},
    utils::{get_evaluated_muli_addi_at_a, get_folded_claim_sum, get_folded_polys},
};
use ark_ec::pairing::Pairing;
use ark_ff::{BigInteger, PrimeField};
use polynomials::multilinear::multilinear_poly::MultilinearPoly;
use sha3::Keccak256;
use std::marker::PhantomData;
use sumcheck::{fiat_shamir::FiatShamir, sumcheck_protocol::partial_verify};

pub struct GKRVerifier<F: PrimeField, P: Pairing> {
    _phantom: PhantomData<F>,
    _phantom_p: PhantomData<P>,
}

impl<F: PrimeField, P: Pairing> GKRVerifier<F, P> {
    pub fn verify(
        input_layer: &[F],
        circuit: &mut Circuit<F>,
        transcript: &mut FiatShamir<Keccak256, F>,
        proof: GKRProof<F>,
    ) -> bool {
        let layer_count = circuit.get_layer_count();
        let length_of_rs = proof.output_poly.n_vars;

        transcript.absorb(&proof.output_poly.to_bytes());

        let mut random_values: Vec<F> = transcript.squeeze_n(length_of_rs);

        for layer_i in 0..layer_count {
            let (muli_a_b_c, addi_a_b_c) = (
                circuit.add_mul_i(layer_i, Op::MUL),
                circuit.add_mul_i(layer_i, Op::ADD),
            );

            let (mut new_muli_b_c, mut new_addi_b_c) = match layer_i {
                0 => get_evaluated_muli_addi_at_a(muli_a_b_c, addi_a_b_c, random_values.to_vec()),
                _ => {
                    let (alpha, beta) = (transcript.squeeze(), transcript.squeeze());

                    let (new_muli_b_c, new_addi_b_c) = get_folded_polys(
                        &alpha,
                        &beta,
                        muli_a_b_c,
                        addi_a_b_c,
                        &random_values[0..random_values.len() / 2],
                        &random_values[random_values.len() / 2..],
                    );

                    (new_muli_b_c, new_addi_b_c)
                }
            };

            let (challenges, claimed_sum) =
                partial_verify(&proof.sumcheck_proofs[layer_i], transcript);

            let (new_muli_b_c_eval, new_addi_b_c_eval) = (
                new_muli_b_c.evaluate(challenges.to_vec()),
                new_addi_b_c.evaluate(challenges.to_vec()),
            );

            let (next_w_i_b_eval, next_w_i_c_eval) = if layer_i + 1 == layer_count {
                let (r_b, r_c) = challenges.split_at(challenges.len() / 2);

                let mut next_w_i = MultilinearPoly::new(
                    input_layer.to_vec(),
                    input_layer.len().ilog2().try_into().unwrap(),
                );

                (
                    next_w_i.evaluate(r_b.to_vec()),
                    next_w_i.evaluate(r_c.to_vec()),
                )
            } else {
                proof.w_poly_evals[layer_i]
            };

            transcript.absorb_n(&[
                &next_w_i_b_eval.into_bigint().to_bytes_le(),
                &next_w_i_c_eval.into_bigint().to_bytes_le(),
            ]);

            let fbc_eval = (new_addi_b_c_eval * (next_w_i_b_eval + next_w_i_c_eval))
                + (new_muli_b_c_eval * (next_w_i_b_eval * next_w_i_c_eval));

            if fbc_eval != claimed_sum {
                return false;
            }

            random_values = challenges;
        }

        true
    }
}
