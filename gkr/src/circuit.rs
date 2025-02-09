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

#[derive(Clone, Debug)]
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

        for layer in &self.layers {
            dbg!(&layer);
            let prev_layer = layer_outputs.last().unwrap();
            let mut current_outputs = vec![F::zero(); prev_layer.len() / 2];

            for gate in layer {
                dbg!(&gate);
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

    pub(crate) fn get_layer_poly(&self, layer_id: usize) -> MultilinearPoly<F> {
        todo!("Implement layer_i polynomial conversion");
    }

    pub(crate) fn add_i() {
        todo!()
    }

    pub(crate) fn mul_i() {
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

        let layer_1 = vec![gate_a, gate_b, gate_c, gate_d];
        let layer_2 = vec![gate_e, gate_f];
        let layer_3 = vec![gate_g];

        let circuit = Circuit::<Fr>::new(vec![layer_1, layer_2, layer_3]);

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
}
