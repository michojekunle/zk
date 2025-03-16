use crate::multilinear::multilinear_poly::MultilinearPoly;
use crate::composed::product_poly::ProductPoly;
use ark_ff::PrimeField;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SumPoly<F: PrimeField> {
    pub polys: Vec<ProductPoly<F>>,
}

impl<F: PrimeField> SumPoly<F> {
    pub fn new(polys: Vec<ProductPoly<F>>) -> Self {
        SumPoly { polys }
    }

    pub fn degree(&self) -> i32 {
        self.polys.len() as i32
    }

    pub fn partial_evaluate(&mut self, partial_evals: Vec<Vec<(usize, F)>>) -> Self {
        for (i, p_evals) in partial_evals.iter().enumerate() {
            self.polys[i] = self.polys[i].partial_evaluate(p_evals.to_vec());
        }

        SumPoly::new(self.polys.clone())
    }

    pub fn evaluate(&mut self, values: Vec<Vec<Vec<F>>>) -> F {
        let mut sum: F = F::zero();

        for (i, value) in values.iter().enumerate() {
            let eval = self.polys[i].evaluate(value.to_vec());
            sum += eval;
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_bn254::Fr;

    #[test]
    fn test_sum_poly() {
        // Basic sum poly tests
        let poly1 = MultilinearPoly::new(vec![Fr::from(1), Fr::from(2)], 1);
        let poly2 = MultilinearPoly::new(vec![Fr::from(3), Fr::from(4)], 1);
        let prod1 = ProductPoly::new(vec![poly1, poly2]);

        let poly3 = MultilinearPoly::new(vec![Fr::from(5), Fr::from(6)], 1);
        let poly4 = MultilinearPoly::new(vec![Fr::from(7), Fr::from(8)], 1);
        let prod2 = ProductPoly::new(vec![poly3, poly4]);

        let mut sum_poly = SumPoly::new(vec![prod1, prod2]);

        // Test basic properties
        assert_eq!(sum_poly.polys.len(), 2);

        // Test evaluation
        let values = vec![
            vec![vec![Fr::from(1)], vec![Fr::from(1)]], // For first product poly
            vec![vec![Fr::from(1)], vec![Fr::from(1)]], // For second product poly
        ];
        let result = sum_poly.evaluate(values);
        // First product evaluates to 8 (2*4)
        // Second product evaluates to 48 (6*8)
        // Sum should be 56
        assert_eq!(result, Fr::from(56));
    }

    #[test]
    fn test_empty_sum_poly() {
        // Test empty sum poly
        let sum_poly: SumPoly<Fr> = SumPoly::new(vec![]);
        assert_eq!(sum_poly.polys.len(), 0);

        // Test evaluation of empty sum poly
        let mut empty_sum = SumPoly::<Fr>::new(vec![]);
        let result = empty_sum.evaluate(vec![]);
        assert_eq!(result, Fr::from(0)); // Empty sum should return 0
    }

    #[test]
    fn test_single_term_sum_poly() {
        // Test sum poly with single product term
        let poly1 = MultilinearPoly::new(vec![Fr::from(2), Fr::from(3)], 1);
        let prod = ProductPoly::new(vec![poly1]);
        let mut sum_poly = SumPoly::new(vec![prod]);

        let values = vec![vec![vec![Fr::from(1)]]];
        let result = sum_poly.evaluate(values);
        assert_eq!(result, Fr::from(3)); // Should evaluate to 3 when x=1
    }

    #[test]
    fn test_multiple_terms_sum_poly() {
        // Create three product polys
        let prod1 = ProductPoly::new(vec![MultilinearPoly::new(
            vec![Fr::from(1), Fr::from(2)],
            1,
        )]);

        let prod2 = ProductPoly::new(vec![MultilinearPoly::new(
            vec![Fr::from(3), Fr::from(4)],
            1,
        )]);

        let prod3 = ProductPoly::new(vec![MultilinearPoly::new(
            vec![Fr::from(5), Fr::from(6)],
            1,
        )]);

        let mut sum_poly = SumPoly::new(vec![prod1, prod2, prod3]);

        let values = vec![
            vec![vec![Fr::from(1)]],
            vec![vec![Fr::from(1)]],
            vec![vec![Fr::from(1)]],
        ];

        let result = sum_poly.evaluate(values);
        // When x=1: 2 + 4 + 6 = 12
        assert_eq!(result, Fr::from(12));
    }
}