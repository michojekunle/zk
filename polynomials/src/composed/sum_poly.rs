use crate::composed::product_poly::ProductPoly;
use ark_ff::PrimeField;
use std::iter;

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

    pub fn partial_evaluate(&self, partial_eval: (usize, F)) -> Self {
        let deg: usize = self.degree().try_into().unwrap();

        let new_polys: Vec<ProductPoly<F>> = (0..deg)
            .map(|i| self.polys[i].partial_evaluate(partial_eval))
            .collect();

        SumPoly::new(new_polys)
    }

    pub fn evaluate(&self, values: Vec<F>) -> F {
        let mut sum: F = F::zero();
        let deg: usize = self.degree().try_into().unwrap();

        for i in 0..deg {
            let eval = self.polys[i].evaluate(values.to_vec());
            sum += eval;
        }
        sum
    }

    pub fn reduce(&self) -> Vec<F> {
        let general_poly_length = self.length();
        let reduced_product_polys: Vec<Vec<F>> =
            self.polys.iter().map(|poly| poly.reduce()).collect();

        let res = iter::repeat(())
            .enumerate()
            .map(|(index, _)| {
                let mut running_idx_sum = F::zero();

                reduced_product_polys.iter().for_each(|poly| {
                    running_idx_sum += poly[index];
                });

                running_idx_sum
            })
            .take(general_poly_length)
            .collect();

        res
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.polys.iter().flat_map(|poly| poly.to_bytes()).collect()
    }
    
    pub fn get_poly_length(polys: &[ProductPoly<F>]) -> usize {
        polys.first().unwrap().length()
    }

    pub fn length(&self) -> usize {
        Self::get_poly_length(&self.polys)
    }

    pub fn n_vars(&self) -> u32 {
        self.length().ilog2()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_bn254::Fr;
    use crate::multilinear::multilinear_poly::MultilinearPoly;

    #[test]
    fn test_sum_poly() {
        // Basic sum poly tests
        let poly1 = MultilinearPoly::new(vec![Fr::from(1), Fr::from(2)], 1);
        let poly2 = MultilinearPoly::new(vec![Fr::from(3), Fr::from(4)], 1);
        let prod1 = ProductPoly::new(vec![poly1, poly2]);

        let poly3 = MultilinearPoly::new(vec![Fr::from(5), Fr::from(6)], 1);
        let poly4 = MultilinearPoly::new(vec![Fr::from(7), Fr::from(8)], 1);
        let prod2 = ProductPoly::new(vec![poly3, poly4]);

        let sum_poly = SumPoly::new(vec![prod1, prod2]);

        // Test basic properties
        assert_eq!(sum_poly.polys.len(), 2);

        // Test evaluation
        let values = vec![Fr::from(1)];
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
        let empty_sum = SumPoly::<Fr>::new(vec![]);
        let result = empty_sum.evaluate(vec![]);
        assert_eq!(result, Fr::from(0)); // Empty sum should return 0
    }

    #[test]
    fn test_single_term_sum_poly() {
        // Test sum poly with single product term
        let poly1 = MultilinearPoly::new(vec![Fr::from(2), Fr::from(3)], 1);
        let prod = ProductPoly::new(vec![poly1]);
        let sum_poly = SumPoly::new(vec![prod]);

        let values = vec![Fr::from(1)];
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

        let sum_poly = SumPoly::new(vec![prod1, prod2, prod3]);

        let values = vec![Fr::from(1)];

        let result = sum_poly.evaluate(values);
        // When x=1: 2 + 4 + 6 = 12
        assert_eq!(result, Fr::from(12));
    }
}
