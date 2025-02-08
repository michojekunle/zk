use ark_ff::PrimeField;
use crate::multilinear_poly::MultilinearPoly;

struct Gate {
    left: usize,
    right: usize,
    output: usize,
    op: Op
}

enum Op {
    ADD,
    MUL,
}

struct Layer {
    gates: Vec<Gate>,
}

struct Circuit {
    layers: Vec<Layer>,
}

impl Gate {
    fn new(left: usize, right: usize, output: usize, op: Op) -> Self {
        Gate {
            left,
            right,
            output,
            op
        }
    }

    fn eval(&self) -> u64 {
        
    }
}

impl Layer {
    fn new() -> Self {

    }
}

impl <F: PrimeField>Circuit<F> {
    fn new(layers: Vec<Layer>) -> Self {
        Circuit {
            layers
        }
    }

    fn eval(&self, input_layer: Vec<F>) -> Vec<Vec<F>> {

    }

    fn get_layer_poly(&self, layer_id: usize) -> MultilinearPoly {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gate() {

    }

    #[test]
    fn test_circuit_implementation() {

    }
}