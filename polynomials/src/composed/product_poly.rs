use crate::multilinear_poly::MultilinearPoly;
use ark_ff::PrimeField;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ProductPoly<F: PrimeField> {
    pub(crate) polys: Vec<MultilinearPoly<F>>,
}

impl<F: PrimeField> ProductPoly<F> {
    pub(crate) fn new(polys: Vec<MultilinearPoly<F>>) -> Self {
        ProductPoly { polys }
    }

    pub(crate) fn partial_evaluate(&mut self, partial_evals: Vec<(usize, F)>) -> Self {
        for (i, (pos, val)) in partial_evals.iter().enumerate() {
            self.polys[i] = self.polys[i].partial_evaluate((*pos, *val));
        }

        ProductPoly::new(self.polys.clone())
    }

    pub(crate) fn evaluate(&mut self, values: Vec<Vec<F>>) -> F {
        let mut product: F = F::one();

        for (i, value) in values.iter().enumerate() {
            let eval = self.polys[i].evaluate(value.to_vec());
            product *= eval;
        }

        product
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
        let mut product_poly = ProductPoly::new(vec![poly1, poly2]);

        let partial_evals = vec![(0, Fr::from(1)), (0, Fr::from(1))];
        let evaluated = product_poly.partial_evaluate(partial_evals);

        assert_eq!(evaluated.polys.len(), 2);
    }

    #[test]
    fn test_evaluate() {
        let poly1 = MultilinearPoly::new(vec![Fr::from(1), Fr::from(2)], 1);
        let poly2 = MultilinearPoly::new(vec![Fr::from(3), Fr::from(4)], 1);
        let mut product_poly = ProductPoly::new(vec![poly1, poly2]);

        let values = vec![vec![Fr::from(1)], vec![Fr::from(1)]];
        let result = product_poly.evaluate(values);

        // Result should be 2 * 4 = 8 when evaluating at x = 1
        assert_eq!(result, Fr::from(8));
    }

    #[test]
    fn test_multiple_polynomials() {
        let polys = (0..3)
            .map(|i| MultilinearPoly::new(vec![Fr::from(i as u64), Fr::from(i as u64 + 1)], 1))
            .collect();

        let mut product_poly = ProductPoly::new(polys);
        assert_eq!(product_poly.polys.len(), 3);

        let values = vec![vec![Fr::from(1)]; 3];
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
        let mut product_poly: ProductPoly<Fr> = ProductPoly::new(vec![]);
        let result = product_poly.evaluate(vec![]);
        assert_eq!(result, Fr::from(1)); // Empty product should return 1
    }
}
