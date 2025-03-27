use crate::circuit::{Circuit, Op};
use crate::protocol::{GKRProof, GKRProofWithKZG};
use ark_ec::pairing::Pairing;
use ark_ff::{BigInteger, PrimeField};
use polynomials::{
    composed::{product_poly::ProductPoly, sum_poly::SumPoly},
    multilinear::multilinear_poly::MultilinearPoly,
};
use sha3::Keccak256;
use std::marker::PhantomData;
use sumcheck::{fiat_shamir::FiatShamir, sumcheck_protocol::partial_prove};

pub struct GKRProver<F: PrimeField, P: Pairing> {
    _phantom: PhantomData<F>,
    _phantom_p: PhantomData<P>,
}

impl<F: PrimeField, P: Pairing> GKRProver<F, P> {
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

        let running_layer_poly = circuit.get_layer_poly(0, input_layer.to_vec());

        transcript.absorb(&running_layer_poly.to_bytes());

        let mut random_values: Vec<F> = transcript.squeeze_n(length_of_rs);

        for layer_i in 0..layer_count {
            let (mul_i_a_b_c, add_i_a_b_c) = (
                circuit.add_mul_i(layer_idx, Op::MUL),
                circuit.add_mul_i(layer_idx, Op::ADD),
            );

            let (claimed_sum, new_mul_i_b_c, new_add_i_b_c) = match layer_i {
                0 => {
                    // let (mul_i_b_c, add_i_b_c) = 
                }

                _ => {
                    
                }
            };

        }

        GKRProof {
            output_poly: circuit.get_layer_poly(0, input_layer.to_vec()),
            w_poly_evals,
            sumcheck_proofs,
        }
    }
}
