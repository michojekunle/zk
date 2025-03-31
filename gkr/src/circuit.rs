use ark_ff::PrimeField;
use polynomials::composed::{product_poly::ProductPoly, sum_poly::SumPoly};
use polynomials::multilinear::multilinear_poly::MultilinearPoly;
use std::cmp::max;
use std::marker::PhantomData;

#[derive(Clone, Debug)]
pub(crate) struct Gate {
    left: usize,
    right: usize,
    output: usize,
    op: Op,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Op {
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
            let mut current_outputs = vec![F::zero(); max(prev_layer.len() / 2, 2) as usize];

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

        let len = layer_eval.len();
        assert!(len.is_power_of_two(), "Length must be a power of 2");
        let n = (len as f64).log2() as u32;

        MultilinearPoly::new(layer_eval.to_vec(), n.try_into().unwrap())
    }

    pub(crate) fn add_mul_i(&self, layer_id: usize, op: Op) -> MultilinearPoly<F> {
        let layer = &self.layers[layer_id];
        let l_i_vars = (layer_id as u32).max(1);
        let l_i_plus_1_vars = layer_id as u32 + 1;

        // Calculate n_vars once (total bits = output + left + right)
        let n_vars = (l_i_vars + 2 * l_i_plus_1_vars) as usize;
        let mut evals = vec![F::zero(); 1 << n_vars];
        // dbg!(&n_vars);

        for gate in layer {
            // Format the output, left, and right as binary strings with the specified widths
            let output_binary = format!("{:0width$b}", gate.output, width = l_i_vars as usize);
            let left_binary = format!("{:0width$b}", gate.left, width = l_i_plus_1_vars as usize);
            let right_binary = format!("{:0width$b}", gate.right, width = l_i_plus_1_vars as usize);

            // Combine the binary strings and Convert the combined binary string to a decimal number to be used as the index to input 1 in the array
            let combined_binary = format!("{}{}{}", output_binary, left_binary, right_binary);
            let eval_true_index: usize = usize::from_str_radix(&combined_binary, 2).unwrap();


            if gate.op == op {
                evals[eval_true_index] = F::one();
            }

            // dbg!(&evals);
            // dbg!(&op);
        }

        MultilinearPoly::new(evals, n_vars)
    }

    pub(crate) fn w_add_mul(
        poly_1: &MultilinearPoly<F>,
        poly_2: &MultilinearPoly<F>,
        op: Op,
    ) -> MultilinearPoly<F> {
        let new_nvars = poly_1.n_vars + poly_2.n_vars;
        let mut new_evals = vec![F::zero(); 1 << new_nvars];

        for i in 0..poly_1.evals.len() {
            for j in 0..poly_2.evals.len() {
                let new_evals_i = usize::from_str_radix(
                    &format!("{:0width$b}{:0width$b}", i, j, width = poly_1.n_vars),
                    2,
                )
                .unwrap();
                if op == Op::ADD {
                    new_evals[new_evals_i] = poly_1.evals[i] + poly_2.evals[j]
                } else if op == Op::MUL {
                    new_evals[new_evals_i] = poly_1.evals[i] * poly_2.evals[j]
                };
            }
        }

        MultilinearPoly::new(new_evals, new_nvars)
    }

    pub(crate) fn generate_fbc(
        add_i: MultilinearPoly<F>,
        mul_i: MultilinearPoly<F>,
        w_i_plus_1: &MultilinearPoly<F>,
    ) -> SumPoly<F> {
        let w_add_bc = Self::w_add_mul(w_i_plus_1, w_i_plus_1, Op::ADD);
        let w_mul_bc = Self::w_add_mul(w_i_plus_1, w_i_plus_1, Op::MUL);
        let mut product_polys: Vec<ProductPoly<F>> = Vec::new();

        product_polys.push(ProductPoly::new(vec![add_i, w_add_bc]));
        product_polys.push(ProductPoly::new(vec![mul_i, w_mul_bc]));

        SumPoly::new(product_polys)
    }

    pub(crate) fn get_layer_count(&self) -> usize {
        self.layers.len()
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

        assert_eq!(layer_0_poly.evals.len(), 2);
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

        let circuit_add_0 = circuit.add_mul_i(0, Op::ADD);
        let circuit_add_1 = circuit.add_mul_i(1, Op::ADD);
        let circuit_add_2 = circuit.add_mul_i(2, Op::ADD);

        assert!(
            circuit_add_0.evals.len() == 8,
            "getting add_i for layer_index 0 failed"
        );
        assert!(
            circuit_add_1.evals.len() == 32,
            "getting add_i for layer_index 1 failed"
        );
        assert!(
            circuit_add_2.evals.len() == 256,
            "getting add_i for layer_index 2 failed"
        );
    }

    #[test]
    fn test_mul_i() {
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

        let circuit_mul_0 = circuit.add_mul_i(0, Op::MUL);
        let circuit_mul_1 = circuit.add_mul_i(1, Op::MUL);
        let circuit_mul_2 = circuit.add_mul_i(2, Op::MUL);

        assert!(
            circuit_mul_0.evals.len() == 8,
            "getting mul_i for layer_index 0 failed"
        );
        assert!(
            circuit_mul_1.evals.len() == 32,
            "getting mul_i for layer_index 1 failed"
        );

        // verify mul_2 poly len and points gates
        assert!(
            circuit_mul_2.evals.len() == 256,
            "getting mul_i for layer_index 2 failed"
        );
        assert!(
            circuit_mul_2.evals[usize::from_str_radix("01010011", 2).unwrap()] == Fr::from(1),
            "ensure mul_i for layer_index 2 at 01010011 failed"
        );
        assert!(
            circuit_mul_2.evals[usize::from_str_radix("10100101", 2).unwrap()] == Fr::from(1),
            "ensure mul_i for layer_index 2 at 10100101 failed"
        );
        assert!(
            circuit_mul_2.evals[usize::from_str_radix("11110111", 2).unwrap()] == Fr::from(1),
            "ensure mul_i for layer_index 2 at 11110111 failed"
        );
    }

    #[test]
    fn test_w_add() {
        // Create two simple polynomials
        let poly1 = MultilinearPoly::new(vec![Fr::from(1u64), Fr::from(2u64)], 1);
        let poly2 = MultilinearPoly::new(vec![Fr::from(3u64), Fr::from(4u64)], 1);

        let result = Circuit::<Fr>::w_add_mul(&poly1, &poly2, Op::ADD);
        let result_2 = Circuit::<Fr>::w_add_mul(&result, &result, Op::ADD);

        // Result should have n_vars = 2 and 2^2 = 4 evaluations
        assert_eq!(result.n_vars, 2);
        assert_eq!(result.evals.len(), 4);

        // Result should have n_vars = 4 and 2^4 = 16 evaluations
        assert_eq!(result_2.n_vars, 4);
        assert_eq!(result_2.evals.len(), 16);

        // Check specific evaluation points
        // For binary representation: 00, 01, 10, 11
        assert_eq!(result.evals[0], Fr::from(4u64)); // 1 + 3
        assert_eq!(result.evals[1], Fr::from(5u64)); // 1 + 4
        assert_eq!(result.evals[2], Fr::from(5u64)); // 2 + 3
        assert_eq!(result.evals[3], Fr::from(6u64)); // 2 + 4

        // Check specific evaluation points
        // For binary representation: 0000, 0001, 0010, 0011, 0100, 0101,0110, 0111, 1000, 1001, 1010, 1011, 1100, 1101, 1110, 1111
        assert_eq!(result_2.evals[0], Fr::from(8u64)); // 4 + 4
        assert_eq!(result_2.evals[1], Fr::from(9u64)); // 4 + 5
        assert_eq!(result_2.evals[2], Fr::from(9u64)); // 4 + 5
        assert_eq!(result_2.evals[3], Fr::from(10u64)); // 4 + 6
        assert_eq!(result_2.evals[4], Fr::from(9u64)); // 5 + 4
        assert_eq!(result_2.evals[5], Fr::from(10u64)); // 5 + 5
        assert_eq!(result_2.evals[6], Fr::from(10u64)); // 5 + 5
        assert_eq!(result_2.evals[7], Fr::from(11u64)); // 5 + 6
        assert_eq!(result_2.evals[8], Fr::from(9u64)); // 5 + 4
        assert_eq!(result_2.evals[9], Fr::from(10u64)); // 5 + 5
        assert_eq!(result_2.evals[10], Fr::from(10u64)); // 5 + 5
        assert_eq!(result_2.evals[11], Fr::from(11u64)); // 5 + 6
        assert_eq!(result_2.evals[12], Fr::from(10u64)); // 6 + 4
        assert_eq!(result_2.evals[13], Fr::from(11u64)); // 6 + 5
        assert_eq!(result_2.evals[14], Fr::from(11u64)); // 6 + 5
        assert_eq!(result_2.evals[15], Fr::from(12u64)); // 6 + 6
    }

    #[test]
    fn test_w_mul() {
        // Create two simple polynomials
        let poly1 = MultilinearPoly::new(vec![Fr::from(1u64), Fr::from(2u64)], 1);
        let poly2 = MultilinearPoly::new(vec![Fr::from(3u64), Fr::from(4u64)], 1);

        let result = Circuit::<Fr>::w_add_mul(&poly1, &poly2, Op::MUL);
        let result_2 = Circuit::<Fr>::w_add_mul(&result, &result, Op::MUL);

        // Result should have n_vars = 2 and 2^2 = 4 evaluations
        assert_eq!(result.n_vars, 2);
        assert_eq!(result.evals.len(), 4);

        // Result should have n_vars = 4 and 2^4 = 16 evaluations
        assert_eq!(result_2.n_vars, 4);
        assert_eq!(result_2.evals.len(), 16);

        // Check specific evaluation points
        // For binary representation: 00, 01, 10, 11
        assert_eq!(result.evals[0], Fr::from(3u64)); // 1 * 3
        assert_eq!(result.evals[1], Fr::from(4u64)); // 1 * 4
        assert_eq!(result.evals[2], Fr::from(6u64)); // 2 * 3
        assert_eq!(result.evals[3], Fr::from(8u64)); // 2 * 4

        // Check specific evaluation points
        // For binary representation: 0000, 0001, 0010, 0011, 0100, 0101,0110, 0111, 1000, 1001, 1010, 1011, 1100, 1101, 1110, 1111
        assert_eq!(result_2.evals[0], Fr::from(9u64)); // 3 * 3
        assert_eq!(result_2.evals[1], Fr::from(12u64)); // 3 * 4
        assert_eq!(result_2.evals[2], Fr::from(18u64)); // 3 * 6
        assert_eq!(result_2.evals[3], Fr::from(24u64)); // 3 * 8
        assert_eq!(result_2.evals[4], Fr::from(12u64)); // 4 * 3
        assert_eq!(result_2.evals[5], Fr::from(16u64)); // 4 * 4
        assert_eq!(result_2.evals[6], Fr::from(24u64)); // 4 * 6
        assert_eq!(result_2.evals[7], Fr::from(32u64)); // 4 * 8
        assert_eq!(result_2.evals[8], Fr::from(18u64)); // 6 * 3
        assert_eq!(result_2.evals[9], Fr::from(24u64)); // 6 * 4
        assert_eq!(result_2.evals[10], Fr::from(36u64)); // 6 * 6
        assert_eq!(result_2.evals[11], Fr::from(48u64)); // 6 * 8
        assert_eq!(result_2.evals[12], Fr::from(24u64)); // 8 * 3
        assert_eq!(result_2.evals[13], Fr::from(32u64)); // 8 * 4
        assert_eq!(result_2.evals[14], Fr::from(48u64)); // 8 * 6
        assert_eq!(result_2.evals[15], Fr::from(64u64)); // 8 * 8
    }
}
