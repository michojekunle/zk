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

    fn eval() -> Self {
        todo!()   
    }
}

impl Layer {
    fn new() -> Self {

    }
}

impl <F: PrimeField>Circuit<F> {
    fn new() -> Self {

    }

    fn eval() -> F {

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circuit_implementation() {

    }
}