use crate::circuit::{Circuit, Op};
use crate::{
    protocol::{GKRProof, GKRProofWithKZG},
    utils::{get_evaluated_muli_addi_at_a, get_folded_claim_sum, get_folded_polys},
};
use ark_ec::pairing::Pairing;
use ark_ff::{BigInteger, PrimeField};
use polynomials::{
    composed::{product_poly::ProductPoly, sum_poly::SumPoly},
    multilinear::multilinear_poly::MultilinearPoly,
};
use sha3::Keccak256;
use std::marker::PhantomData;
use sumcheck::{fiat_shamir::FiatShamir, sumcheck_protocol::partial_prove};

pub struct GKRProver<F: PrimeField> {
    _phantom: PhantomData<F>,
}

impl<F: PrimeField> GKRProver<F> {
    pub fn prove(
        input_layer: &[F],
        circuit: &mut Circuit<F>,
        transcript: &mut FiatShamir<Keccak256, F>,
    ) -> GKRProof<F> {
        // get number of layers and initialize vectors for tracking w_poly_evals and sumcheck_proofs
        let layer_count = circuit.get_layer_count();
        let mut w_poly_evals = Vec::with_capacity(layer_count);
        let mut sumcheck_proofs = Vec::with_capacity(layer_count);

        // legnth of rs for generating random values for the ouput_poly
        let length_of_rs = circuit.get_layer_poly(0, input_layer.to_vec()).n_vars;

        let mut running_layer_poly = circuit.get_layer_poly(0, input_layer.to_vec());

        transcript.absorb(&running_layer_poly.to_bytes());

        let mut random_values: Vec<F> = transcript.squeeze_n(length_of_rs);

        for layer_i in 0..layer_count {
            let (muli_a_b_c, addi_a_b_c) = (
                circuit.add_mul_i(layer_i, Op::MUL),
                circuit.add_mul_i(layer_i, Op::ADD),
            );

            let (claimed_sum, new_muli_b_c, new_addi_b_c) = match layer_i {
                0 => {
                    let (muli_b_c, addi_b_c) = get_evaluated_muli_addi_at_a(
                        muli_a_b_c,
                        addi_a_b_c,
                        random_values.to_vec(),
                    );
                    (
                        running_layer_poly.evaluate(random_values.to_vec()),
                        muli_b_c,
                        addi_b_c,
                    )
                }

                _ => {
                    let (r_b, r_c) = (
                        &random_values[0..random_values.len() / 2],
                        &random_values[random_values.len() / 2..],
                    );

                    dbg!(&r_b);
                    dbg!(&r_c);

                    let w_i_b_eval = running_layer_poly.clone().evaluate(r_b.to_vec());
                    let w_i_c_eval = running_layer_poly.clone().evaluate(r_c.to_vec());

                    dbg!(&w_i_b_eval);
                    dbg!(&w_i_c_eval);

                    transcript.absorb_n(&[
                        &w_i_b_eval.into_bigint().to_bytes_le(),
                        &w_i_c_eval.into_bigint().to_bytes_le(),
                    ]);

                    let (alpha, beta) = (transcript.squeeze(), transcript.squeeze());

                    // get new claim sums, new addi and muli polys, alongside evaluation of current w_i layer poly
                    let (new_muli_b_c, new_addi_b_c) =
                        get_folded_polys(&alpha, &beta, muli_a_b_c, addi_a_b_c, r_b, r_c);

                    w_poly_evals.push((w_i_b_eval, w_i_c_eval));

                    (
                        get_folded_claim_sum(&alpha, &beta, &w_i_b_eval, &w_i_c_eval),
                        new_muli_b_c,
                        new_addi_b_c,
                    )
                }
            };

            let next_w_i = circuit.get_layer_poly(layer_i + 1, input_layer.to_vec());

            dbg!(&next_w_i);
            dbg!(&new_addi_b_c);
            dbg!(&new_muli_b_c);

            let f_bc: SumPoly<F> = Circuit::generate_fbc(new_addi_b_c, new_muli_b_c, &next_w_i);

            dbg!(&f_bc);

            let sumcheck_proof = partial_prove(&f_bc, claimed_sum, transcript);

            dbg!(&sumcheck_proof);

            random_values = sumcheck_proof
                .rand_challenges
                .iter()
                .map(|chal| *chal)
                .collect();
            running_layer_poly = next_w_i;

            sumcheck_proofs.push(sumcheck_proof);
        }

        GKRProof {
            output_poly: circuit.get_layer_poly(0, input_layer.to_vec()),
            w_poly_evals,
            sumcheck_proofs,
        }
    }
}
