use crate::multilinear_poly::MultilinearPoly;
use ark_ff::PrimeField;
use std::marker::PhantomData;

#[derive(Clone, Debug)]
pub(crate) struct Gate {
    left: usize,
    right: usize,
    output: usize,
    op: Op,
}

#[derive(Clone, Debug, PartialEq)]
enum Op {
    ADD,
    MUL,
}

#[derive(Debug)]
pub struct Circuit<F: PrimeField> {
    layers: Vec<Vec<Gate>>,
    _phantom: PhantomData<F>,
}

impl Gate {
    pub(crate) fn new(left: usize, right: usize, output: usize, op: Op) -> Self {
        Gate {
            left,
            right,
            output,
            op,
        }
    }
}

impl<F: PrimeField> Circuit<F> {
    pub(crate) fn new(layers: Vec<Vec<Gate>>) -> Self {
        Circuit {
            layers,
            _phantom: PhantomData,
        }
    }

    pub(crate) fn eval(&self, input_layer: Vec<F>) -> Vec<Vec<F>> {
        let mut layer_outputs = vec![input_layer];

        for layer in self.layers.clone().into_iter().rev() {
            // dbg!(&layer);
            let prev_layer = layer_outputs.last().unwrap();
            let mut current_outputs = vec![F::zero(); prev_layer.len() / 2];

            for gate in layer {
                // dbg!(&gate);
                let result = match gate.op {
                    Op::ADD => prev_layer[gate.left] + prev_layer[gate.right],
                    Op::MUL => prev_layer[gate.left] * prev_layer[gate.right],
                };
                current_outputs[gate.output] = result;
            }

            layer_outputs.push(current_outputs);
        }

        layer_outputs
    }

    pub(crate) fn get_layer_poly(
        &self,
        layer_id: usize,
        input_layer: Vec<F>,
    ) -> MultilinearPoly<F> {
        let evals: Vec<Vec<F>> = self.eval(input_layer).into_iter().rev().collect();

        let layer_eval = &evals[layer_id];

        let N = layer_eval.len();
        assert!(N.is_power_of_two(), "Length must be a power of 2");
        let n = (N as f64).log2() as u32;

        MultilinearPoly::new(layer_eval.to_vec(), n.try_into().unwrap())
    }

    pub(crate) fn add_i(&self, layer_id: usize) -> MultilinearPoly<F> {
        let layer = &self.layers[layer_id];
        let l_i_vars = layer_id as u32;
        let l_i_plus_1_vars = layer_id as u32 + 1;

        let n_vars: usize;
        let add_i_evals: Vec<F>;

        for gate in layer {
            if gate.op == Op::ADD {
                // Format the output, left, and right as binary strings with the specified widths
                let output_binary = format!("{:0width$b}", gate.output, width = l_i_vars as usize);
                let left_binary =
                    format!("{:0width$b}", gate.left, width = l_i_plus_1_vars as usize);
                let right_binary =
                    format!("{:0width$b}", gate.right, width = l_i_plus_1_vars as usize);

                // Combine the binary strings
                let combined_binary = format!("{}{}{}", output_binary, left_binary, right_binary);
                let eval_pow = combined_binary.len();

                // Convert the combined binary string to a decimal number to be used as the index to input 1 in the array
                let eval_true_index = u32::from_str_radix(&combined_binary, 2).unwrap();
                // if
                // if (add_i_evals )
                // add_i_evals
            }
        }

        MultilinearPoly::new(Vec::<F>::new(), 0)
    }

    pub(crate) fn mul_i(&self, layer_id: usize) -> MultilinearPoly<F> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_bn254::Fr;

    #[test]
    fn test_circuit_implementation() {
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

        let circuit = Circuit::<Fr>::new(vec![layer_0, layer_1, layer_2]);

        let input = vec![
            Fr::from(1u64),
            Fr::from(2u64),
            Fr::from(3u64),
            Fr::from(4u64),
            Fr::from(5u64),
            Fr::from(6u64),
            Fr::from(7u64),
            Fr::from(8u64),
        ];
        let outputs = circuit.eval(input);
        dbg!(&outputs);
        assert_eq!(outputs.len(), 4);
    }

    #[test]
    fn test_get_layer_poly() {
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

        let circuit = Circuit::<Fr>::new(vec![layer_0, layer_1, layer_2]);

        let input = vec![
            Fr::from(1u64),
            Fr::from(2u64),
            Fr::from(3u64),
            Fr::from(4u64),
            Fr::from(5u64),
            Fr::from(6u64),
            Fr::from(7u64),
            Fr::from(8u64),
        ];

        let layer_0_poly = circuit.get_layer_poly(0, input.clone());
        let layer_1_poly = circuit.get_layer_poly(1, input.clone());
        let layer_2_poly = circuit.get_layer_poly(2, input);

        dbg!(&layer_0_poly);
        dbg!(&layer_1_poly);
        dbg!(&layer_2_poly);

        assert_eq!(layer_0_poly.evals.len(), 1);
        assert_eq!(layer_1_poly.evals.len(), 2);
        assert_eq!(layer_2_poly.evals.len(), 4);
    }

    #[test]
    fn test_add_i() {
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

        let circuit = Circuit::<Fr>::new(vec![layer_0, layer_1, layer_2]);

        circuit.add_i(1);
    }

    #[test]
    fn test_mul_i() {}
}
