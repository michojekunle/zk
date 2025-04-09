use crate::multilinear::multilinear_poly::MultilinearPoly;
use ark_ff::PrimeField;
use std::iter;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProductPoly<F: PrimeField> {
    pub polys: Vec<MultilinearPoly<F>>,
}

impl<F: PrimeField> ProductPoly<F> {
    pub fn new(polys: Vec<MultilinearPoly<F>>) -> Self {
        ProductPoly { polys }
    }

    pub fn degree(&self) -> i32 {
        self.polys.len() as i32
    }

    pub fn partial_evaluate(&self, (pos, val): (usize, F)) -> Self {
        let deg: usize = self.degree().try_into().unwrap();

        let new_polys: Vec<MultilinearPoly<F>> = (0..deg)
            .map(|i| self.polys[i].partial_evaluate((pos, val)))
            .collect();

        ProductPoly::new(new_polys)
    }

    pub fn evaluate(&self, values: Vec<F>) -> F {
        let mut product: F = F::one();
        let deg: usize = self.degree().try_into().unwrap();

        for i in 0..deg {
            let eval = self.polys[i].evaluate(values.to_vec());
            product *= eval;
        }

        product
    }

    pub fn reduce(&self) -> Vec<F> {
        // perform element-wise product on each multilinear polynomial
        let general_poly_length = self.length();

        let res = iter::repeat(())
            .enumerate()
            .map(|(index, _)| {
                let mut running_idx_prod = F::one();

                self.polys.iter().for_each(|poly| {
                    running_idx_prod *= poly.evals[index];
                });

                running_idx_prod
            })
            .take(general_poly_length)
            .collect();

        res
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.polys.iter().flat_map(|poly| poly.to_bytes()).collect()
    }

    pub fn get_poly_length(polys: &Vec<MultilinearPoly<F>>) -> usize {
        polys.first().unwrap().evals.len()
    }

    pub fn length(&self) -> usize {
        Self::get_poly_length(&self.polys)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_bn254::Fr;

    #[test]
    fn test_new() {
        let poly1 = MultilinearPoly::new(vec![Fr::from(1), Fr::from(2)], 1);
        let poly2 = MultilinearPoly::new(vec![Fr::from(3), Fr::from(4)], 1);
        let product_poly = ProductPoly::new(vec![poly1, poly2]);

        assert_eq!(product_poly.polys.len(), 2);
    }

    #[test]
    fn test_partial_evaluate() {
        let poly1 = MultilinearPoly::new(vec![Fr::from(1), Fr::from(2)], 1);
        let poly2 = MultilinearPoly::new(vec![Fr::from(3), Fr::from(4)], 1);
        let product_poly = ProductPoly::new(vec![poly1, poly2]);

        let partial_evals = (0, Fr::from(1));
        let evaluated = product_poly.partial_evaluate(partial_evals);

        assert_eq!(evaluated.polys.len(), 2);
    }

    #[test]
    fn test_evaluate() {
        let poly1 = MultilinearPoly::new(vec![Fr::from(1), Fr::from(2)], 1);
        let poly2 = MultilinearPoly::new(vec![Fr::from(3), Fr::from(4)], 1);
        let product_poly = ProductPoly::new(vec![poly1, poly2]);

        let values = vec![Fr::from(1)];
        let result = product_poly.evaluate(values);

        // Result should be 2 * 4 = 8 when evaluating at x = 1
        assert_eq!(result, Fr::from(8));
    }

    #[test]
    fn test_multiple_polynomials() {
        let polys = (0..3)
            .map(|i| MultilinearPoly::new(vec![Fr::from(i as u64), Fr::from(i as u64 + 1)], 1))
            .collect();

        let product_poly = ProductPoly::new(polys);
        assert_eq!(product_poly.polys.len(), 3);

        let values = vec![Fr::from(1)];
        let result = product_poly.evaluate(values);
        // Result should be 1 * 2 * 3 = 6 when evaluating at x = 1
        assert_eq!(result, Fr::from(6));
    }

    #[test]
    fn test_empty_product_poly() {
        let product_poly: ProductPoly<Fr> = ProductPoly::new(vec![]);
        assert_eq!(product_poly.polys.len(), 0);
    }

    #[test]
    fn test_empty_evaluation() {
        let product_poly: ProductPoly<Fr> = ProductPoly::new(vec![]);
        let result = product_poly.evaluate(vec![]);
        assert_eq!(result, Fr::from(1)); // Empty product should return 1
    }
}
