use ark_ff::{BigInteger, PrimeField};
use std::ops::Add;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MultilinearPoly<F: PrimeField> {
    pub n_vars: usize,
    pub evals: Vec<F>,
}

impl<F: PrimeField> MultilinearPoly<F> {
    pub fn new(evals: Vec<F>, n_vars: usize) -> Self {
        MultilinearPoly { evals, n_vars }
    }

    pub fn partial_evaluate(&mut self, (pos, val): (usize, F)) -> Self {
        let length = self.evals.len();

        // dbg!(&self.evals);
        // dbg!(&length);
        // dbg!(1 << (pos + 1));
        // dbg!(&pos);
        // dbg!(&val);
        // println!();
        // println!();

        if self.n_vars == 0 {
            return MultilinearPoly::new(self.evals.to_vec(), self.n_vars)
        }

        if self.n_vars > 1 && 1 << (pos + 1) > length as i32 {
            panic!(
                "The position is out of range for this polynomial with {} evals",
                self.evals.len()
            );
        }

        let mut new_evals = Vec::with_capacity(length / 2);

        let unique_pairs_evals = Self::get_unique_pairs_evals(&self.evals, pos);

        new_evals.extend(unique_pairs_evals.iter().map(|(c_i, c_pair_index)| {
            *c_i + val * (*c_pair_index - c_i)
        }));

        dbg!(&self.n_vars);

        MultilinearPoly::new(new_evals, self.n_vars - 1)
    }

    pub fn evaluate(&mut self, values: Vec<F>) -> F {
        // dbg!(&values);
        for i in 0..values.len() {
            // dbg!(&i);
            // dbg!(&self.n_vars);
            // dbg!(&self.evals);
            *self = self.partial_evaluate((self.n_vars - 1, values[i]));
        }

        // dbg!(&self);
        self.evals[0]
    }

    pub fn scalar_mul(&mut self, scalar: F) -> Self {
        let new_evals = self.evals.iter().map(|e| {
            scalar * *e
        }).collect();

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

     // Adds two polynomials of same variables together
     pub fn _add(&self, other: &MultilinearPoly<F>) -> Self {
        if self.n_vars != other.n_vars {
            panic!("Polynomial must have the same length");
        };

        let mut new_evals = vec![F::zero(); other.evals.len()];

        (0..self.evals.len()).for_each(|idx| {
            new_evals[idx] +=
                self.evals[idx] + other.evals[idx];
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
        let mut poly = MultilinearPoly::<Fq> {
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
        let mut poly = MultilinearPoly::<Fq> {
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
        let mut poly_2 = MultilinearPoly::new(
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
        let mut poly_2 = MultilinearPoly::new(
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
        let mut poly_2 = MultilinearPoly::new(
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
        let mut poly_2 = MultilinearPoly::new(
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
        let mut poly = MultilinearPoly::new(
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
}
