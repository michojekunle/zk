use ark_ff::{BigInteger, PrimeField};
use std::ops::Add;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MultilinearPoly<F: PrimeField> {
    pub n_vars: usize,
    pub evals: Vec<F>,
}

pub enum BlowUpDirection {
    Left,
    Right,
}

impl<F: PrimeField> MultilinearPoly<F> {
    pub fn new(evals: Vec<F>, n_vars: usize) -> Self {
        MultilinearPoly { evals, n_vars }
    }

    pub fn partial_evaluate(&self, (pos, val): (usize, F)) -> Self {
        let length = self.evals.len();

        if self.n_vars == 0 {
            return MultilinearPoly::new(self.evals.to_vec(), self.n_vars);
        }

        if self.n_vars > 1 && 1 << (pos + 1) > length as i32 {
            panic!(
                "The position is out of range for this polynomial with {} evals",
                self.evals.len()
            );
        }

        let mut new_evals = Vec::with_capacity(length / 2);

        let unique_pairs_evals = Self::get_unique_pairs_evals(&self.evals, pos);

        new_evals.extend(
            unique_pairs_evals
                .iter()
                .map(|(c_i, c_pair)| *c_i + val * (*c_pair - c_i)),
        );

        MultilinearPoly::new(new_evals, self.n_vars - 1)
    }

    pub fn evaluate(&self, values: Vec<F>) -> F {
        let mut poly = self.clone();

        for i in 0..values.len() {
            poly = poly.partial_evaluate((poly.n_vars - 1, values[i]));
        }

        poly.evals[0]
    }

    pub fn scalar_mul(&self, scalar: F) -> Self {
        let new_evals = self.evals.iter().map(|e| scalar * *e).collect();

        MultilinearPoly::new(new_evals, self.n_vars)
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        // Convert evaluation points to a serializable format (e.g., bytes)
        let serializable_points: Vec<u8> = self
            .evals
            .iter()
            .flat_map(|point| point.into_bigint().to_bytes_le())
            .collect();
        serializable_points
    }

    fn get_unique_pairs_evals(arr: &Vec<F>, pos: usize) -> Vec<(F, F)> {
        let mask = 1 << pos; // Mask for the current bit position
        let mut evals = Vec::new(); // To store unique pair evals

        for i in 0..arr.len() {
            let pair = i ^ mask; // Calculate the pair index by flipping the bit at `pos`

            // Only process unique pairs (avoid duplicates)
            if i < pair {
                evals.push((arr[i], arr[pair])); // Store evals as pairs
            }
        }
        evals
    }

    pub fn compute_quotient_remainder(&self, divisor: &F, pos: usize) -> (Vec<F>, Self) {
        let unique_pairs_evals = Self::get_unique_pairs_evals(&self.evals, pos);

        let remainder = self.partial_evaluate((pos, *divisor));
        let quotient = unique_pairs_evals
            .iter()
            .map(|(c_i, c_pair)| *c_pair - *c_i)
            .collect();

        (quotient, remainder)
    }

    pub fn blow_up_n_times(direction: BlowUpDirection, evals: &Vec<F>, n: usize) -> Vec<F> {
        let mut new_evals = evals.clone();

        for _ in 0..n {
            let mut temp_evals = Vec::with_capacity(new_evals.len() * 2);

            match direction {
                BlowUpDirection::Left => {
                    temp_evals.extend_from_slice(&new_evals);
                    temp_evals.extend_from_slice(&new_evals);
                }
                BlowUpDirection::Right => {
                    for &eval in new_evals.iter() {
                        temp_evals.push(eval); // Duplicate the eval in the first position
                        temp_evals.push(eval); // Retain the original eval in the second position
                    }
                }
            }

            new_evals = temp_evals;
        }

        new_evals
    }

    // Adds two polynomials of same variables together
    pub fn _add(&self, other: &MultilinearPoly<F>) -> Self {
        if self.n_vars != other.n_vars {
            panic!("Polynomial must have the same length");
        };

        let mut new_evals = vec![F::zero(); other.evals.len()];

        (0..self.evals.len()).for_each(|idx| {
            new_evals[idx] += self.evals[idx] + other.evals[idx];
        });

        Self::new(new_evals, self.n_vars)
    }
}

impl<F: PrimeField> Add for MultilinearPoly<F> {
    type Output = Self;

    fn add(self, other: MultilinearPoly<F>) -> Self {
        self._add(&other)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use ark_bn254::{Fq, Fr};

    pub fn to_field(input: Vec<u64>) -> Vec<Fr> {
        input.iter().map(|v| Fr::from(*v)).collect()
    }

    #[test]
    fn test_partial_evaluate_multilinear_polynomial_a_2v() {
        let poly = MultilinearPoly::<Fq> {
            evals: vec![Fq::from(0), Fq::from(2), Fq::from(0), Fq::from(5)],
            n_vars: 2,
        };

        let partial_evaluated_poly = poly.partial_evaluate((1, Fq::from(5)));
        assert_eq!(
            partial_evaluated_poly.evals,
            vec![Fq::from(0), Fq::from(17)]
        );
    }

    #[test]
    fn test_partial_evaluate_multilinear_polynomial_b_2v() {
        let poly = MultilinearPoly::<Fq> {
            evals: vec![Fq::from(0), Fq::from(2), Fq::from(0), Fq::from(5)],
            n_vars: 2,
        };

        let partial_evaluated_poly = poly.partial_evaluate((0, Fq::from(3)));
        assert_eq!(
            partial_evaluated_poly.evals,
            vec![Fq::from(6), Fq::from(15)]
        );
    }

    #[test]
    fn test_partial_evaluate_multilinear_polynomial_a_3v() {
        let poly_2 = MultilinearPoly::new(
            vec![
                Fq::from(0),
                Fq::from(0),
                Fq::from(0),
                Fq::from(3),
                Fq::from(0),
                Fq::from(0),
                Fq::from(2),
                Fq::from(5),
            ],
            3,
        );
        let result = poly_2.partial_evaluate((2, Fq::from(1)));
        assert_eq!(
            result.evals,
            vec![Fq::from(0), Fq::from(0), Fq::from(2), Fq::from(5)]
        );
    }

    #[test]
    fn test_partial_evaluate_multilinear_polynomial_b_3v() {
        let poly_2 = MultilinearPoly::new(
            vec![
                Fq::from(0),
                Fq::from(0),
                Fq::from(0),
                Fq::from(3),
                Fq::from(0),
                Fq::from(0),
                Fq::from(2),
                Fq::from(5),
            ],
            3,
        );
        let result = poly_2.partial_evaluate((1, Fq::from(5)));
        assert_eq!(
            result.evals,
            vec![Fq::from(0), Fq::from(15), Fq::from(10), Fq::from(25)]
        );
    }

    #[test]
    fn test_partial_evaluate_multilinear_polynomial_c_3v() {
        let poly_2 = MultilinearPoly::new(
            vec![
                Fq::from(0),
                Fq::from(0),
                Fq::from(0),
                Fq::from(3),
                Fq::from(0),
                Fq::from(0),
                Fq::from(2),
                Fq::from(5),
            ],
            3,
        );
        let result = poly_2.partial_evaluate((0, Fq::from(3)));
        assert_eq!(
            result.evals,
            vec![Fq::from(0), Fq::from(9), Fq::from(0), Fq::from(11)]
        );
    }

    #[test]
    fn test_evaluate_multilinear_polynomial_abc() {
        let poly_2 = MultilinearPoly::new(
            vec![
                Fq::from(0),
                Fq::from(0),
                Fq::from(0),
                Fq::from(3),
                Fq::from(0),
                Fq::from(0),
                Fq::from(2),
                Fq::from(5),
            ],
            3,
        );
        let result = poly_2.evaluate(vec![Fq::from(1), Fq::from(5), Fq::from(3)]);
        assert_eq!(result, Fq::from(55));
    }

    #[test]
    fn test_scalar_mul() {
        let poly = MultilinearPoly::new(
            vec![
                Fq::from(0),
                Fq::from(0),
                Fq::from(0),
                Fq::from(3),
                Fq::from(0),
                Fq::from(0),
                Fq::from(2),
                Fq::from(5),
            ],
            3,
        );

        let new_poly = poly.scalar_mul(Fq::from(3));

        assert_eq!(new_poly.evals[3], Fq::from(9));
        assert_eq!(new_poly.evals[6], Fq::from(6));
        assert_eq!(new_poly.evals[7], Fq::from(15));
    }

    #[test]
    fn test_blow_up_right() {
        // Single-variable polynomial: 2x
        let evals = to_field(vec![0, 2]); // Evaluations for x = {0, 1}

        // Blow up to two variables (add y as 0y)
        let result = MultilinearPoly::<Fr>::blow_up_n_times(BlowUpDirection::Right, &evals, 1);

        // Expected result: [0, 0, 2, 2]
        assert_eq!(result, to_field(vec![0, 0, 2, 2]));

        // Two-variable polynomial: 2x + 3y
        let evals = to_field(vec![0, 4, 0, 6]);
        let result = MultilinearPoly::<Fr>::blow_up_n_times(BlowUpDirection::Right, &evals, 1);
        dbg!(&result);
        assert_eq!(result, to_field(vec![0, 0, 4, 4, 0, 0, 6, 6]));

        // Three-variable polynomial: 2abc + 3ab + 4bc + 5c
        let evals = to_field(vec![0, 5, 3, 12, 0, 5, 5, 14]); // Evaluations for (a, b, c)

        // Blow up to four variables (add d as 0d)
        let result = MultilinearPoly::<Fr>::blow_up_n_times(BlowUpDirection::Right, &evals, 1);

        // Expected result: [0, 0, 5, 5, 3, 3, 12, 12, 0, 0, 5, 5, 5, 5, 14, 14]
        assert_eq!(
            result,
            to_field(vec![0, 0, 5, 5, 3, 3, 12, 12, 0, 0, 5, 5, 5, 5, 14, 14])
        );
    }

    #[test]
    fn test_blow_up_left() {
        // Single-variable polynomial: 2y
        let evals = to_field(vec![0, 2]); // Evaluations for y = {0, 1}

        // Blow up to two variables (add x as 0x, BlowUpDirection::Left)
        let result = MultilinearPoly::<Fr>::blow_up_n_times(BlowUpDirection::Left, &evals, 1);

        // Expected result: [0, 2, 0, 2]
        assert_eq!(result, to_field(vec![0, 2, 0, 2]));

        let evals = to_field(vec![0, 0, 3, 7]);
        let result = MultilinearPoly::<Fr>::blow_up_n_times(BlowUpDirection::Left, &evals, 1);
        dbg!(&result);
        assert_eq!(result, to_field(vec![0, 0, 3, 7, 0, 0, 3, 7]));

        // Three-variable polynomial: 2bcd + 3bc + 4cd + 5d
        let evals = to_field(vec![0, 5, 0, 9, 0, 5, 3, 14]); // Evaluations for (b, c, d)

        // Blow up to four variables (add a as 0a, BlowUpDirection::Left)
        let result = MultilinearPoly::<Fr>::blow_up_n_times(BlowUpDirection::Left, &evals, 1);

        // Expected result: [0, 0, 5, 5, 3, 3, 12, 12, 0, 0, 5, 5, 5, 5, 14, 14]
        assert_eq!(
            result,
            to_field(vec![0, 5, 0, 9, 0, 5, 3, 14, 0, 5, 0, 9, 0, 5, 3, 14])
        );
    }
}
