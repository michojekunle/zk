use ark_ff::PrimeField;

#[derive(Debug, PartialEq, Eq)]
struct MultilinearPoly<F: PrimeField> {
    coefficients: Vec<F>,
}

impl<F: PrimeField> MultilinearPoly<F> {
    fn new(coefficients: Vec<F>) -> Self {
        MultilinearPoly { coefficients }
    }

    fn partial_evaluate(&self, (pos, val): (usize, F)) -> Self {
        let length = self.coefficients.len();
        if 2_i32.pow(pos as u32 + 1u32) > length as i32 {
            panic!(
                "The position is out of range for this polynomial with {} coefficients",
                self.coefficients.len()
            );
        }

        let mut new_coefficients = vec![F::zero(); (&length / 2).try_into().unwrap()];

        let unique_pairs_coefficients = Self::get_unique_pairs_coefficients(self.coefficients.clone(), pos);
        println!(
            "Coefficients of Unique Pairs: {:?}",
            unique_pairs_coefficients
        );

        for (i, (c_i, c_pair_index)) in unique_pairs_coefficients.iter().enumerate() {
            new_coefficients[i] = *c_i + val * (*c_pair_index - c_i);
        }

        MultilinearPoly::new(new_coefficients)
    }

    fn evaluate(&self, values: Vec<(usize, F)>) -> F {
        let mut partial_evaluated_poly = self.clone();

        for (pos, val) in values {
            partial_evaluated_poly = partial_evaluated_poly.partial_evaluate((pos, val));
        }

        partial_evaluated_poly.coefficients[0]
    }

    fn get_unique_pairs_coefficients(arr: Vec<F>, pos: usize) -> Vec<(F, F)> {
        let mask = 1 << pos; // Mask for the current bit position
        let mut coefficients = Vec::new(); // To store unique pair coefficients

        for i in 0..arr.len() {
            let pair = i ^ mask; // Calculate the pair index by flipping the bit at `pos`

            // Only process unique pairs (avoid duplicates)
            if i < pair {
                println!(
                    "Unique Pair: (i={}, pair={}) -> Values: ({}, {})",
                    i, pair, arr[i], arr[pair]
                );
                coefficients.push((arr[i], arr[pair])); // Store coefficients as pairs
            }
        }

        coefficients
    }
}

fn main() {
    fn get_unique_pairs_coefficients(arr: Vec<i32>, pos: usize) -> Vec<(i32, i32)> {
        let mask = 1 << pos; // Mask for the current bit position
        let mut coefficients = Vec::new(); // To store unique pair coefficients

        for i in 0..arr.len() {
            let pair = i ^ mask; // Calculate the pair index by flipping the bit at `pos`

            // Only process unique pairs (avoid duplicates)
            if i < pair {
                println!(
                    "Unique Pair: (i={}, pair={}) -> Values: ({}, {})",
                    i, pair, arr[i], arr[pair]
                );
                coefficients.push((arr[i], arr[pair])); // Store coefficients as pairs
            }
        }

        coefficients
    }

    // Example usage
    let arr = vec![0, 0, 0, 3, 0, 0, 2, 5];
    let pos = 1;

    let unique_pairs_coefficients = get_unique_pairs_coefficients(arr, pos);
    println!(
        "Coefficients of Unique Pairs: {:?}",
        unique_pairs_coefficients
    );
}

#[cfg(test)]
mod test {
    use super::*;
    use ark_bn254::Fq;

    #[test]
    fn test_partial_evaluate_multilinear_polynomial_a_2v() {
        let poly = MultilinearPoly::<Fq> {
            coefficients: vec![Fq::from(0), Fq::from(2), Fq::from(0), Fq::from(5)],
        };

        let partial_evaluated_poly = poly.partial_evaluate((1, Fq::from(5)));
        assert_eq!(
            partial_evaluated_poly.coefficients,
            vec![Fq::from(0), Fq::from(17)]
        );
    }

    #[test]
    fn test_partial_evaluate_multilinear_polynomial_b_2v() {
        let poly = MultilinearPoly::<Fq> {
            coefficients: vec![Fq::from(0), Fq::from(2), Fq::from(0), Fq::from(5)],
        };

        let partial_evaluated_poly = poly.partial_evaluate((0, Fq::from(3)));
        assert_eq!(
            partial_evaluated_poly.coefficients,
            vec![Fq::from(6), Fq::from(15)]
        );
    }

    #[test]
    fn test_partial_evaluate_multilinear_polynomial_a_3v() {
        let poly_2 = MultilinearPoly::new(vec![
            Fq::from(0),
            Fq::from(0),
            Fq::from(0),
            Fq::from(3),
            Fq::from(0),
            Fq::from(0),
            Fq::from(2),
            Fq::from(5),
        ]);
        let result = poly_2.partial_evaluate((2, Fq::from(1)));
        assert_eq!(
            result.coefficients,
            vec![Fq::from(0), Fq::from(0), Fq::from(2), Fq::from(5)]
        );
    }

    #[test]
    fn test_partial_evaluate_multilinear_polynomial_b_3v() {
        let poly_2 = MultilinearPoly::new(vec![
            Fq::from(0),
            Fq::from(0),
            Fq::from(0),
            Fq::from(3),
            Fq::from(0),
            Fq::from(0),
            Fq::from(2),
            Fq::from(5),
        ]);
        let result = poly_2.partial_evaluate((1, Fq::from(5)));
        assert_eq!(
            result.coefficients,
            vec![Fq::from(0), Fq::from(15), Fq::from(10), Fq::from(25)]
        );
    }

    #[test]
    fn test_partial_evaluate_multilinear_polynomial_c_3v() {
        let poly_2 = MultilinearPoly::new(vec![
            Fq::from(0),
            Fq::from(0),
            Fq::from(0),
            Fq::from(3),
            Fq::from(0),
            Fq::from(0),
            Fq::from(2),
            Fq::from(5),
        ]);
        let result = poly_2.partial_evaluate((0, Fq::from(3)));
        assert_eq!(
            result.coefficients,
            vec![Fq::from(0), Fq::from(9), Fq::from(0), Fq::from(11)]
        );
    }

    #[test]
    fn test_evaluate_multilinear_polynomial_abc() {
        let poly_2 = MultilinearPoly::new(vec![
            Fq::from(0),
            Fq::from(0),
            Fq::from(0),
            Fq::from(3),
            Fq::from(0),
            Fq::from(0),
            Fq::from(2),
            Fq::from(5),
        ]);
        let result = poly_2.evaluate(vec![(2, Fq::from(1)), (1, Fq::from(5)), (0, Fq::from(3))]);
        assert_eq!(
            result,
            Fq::from(55)
        );
    }
}
