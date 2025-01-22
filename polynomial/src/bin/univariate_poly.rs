use ark_ff::PrimeField;
use std::iter::{Product, Sum};
use std::ops::{Add, Mul};

#[derive(Debug, PartialEq, Clone)]
struct UnivariatePoly<F: PrimeField> {
    coefficients: Vec<F>,
}

impl<F: PrimeField> UnivariatePoly<F> {
    fn new(coefficients: Vec<F>) -> Self {
        UnivariatePoly { coefficients }
    }

    fn degree(&self) -> i32 {
        self.coefficients.len() as i32 - 1
    }

    fn evaluate(&self, x: F) -> F {
        self.coefficients
            .iter()
            .rev()
            .cloned()
            .reduce(|acc, curr| acc * x + curr)
            .unwrap()
    }

    fn interpolate(xs: Vec<F>, ys: Vec<F>) -> Self {
        xs.iter()
            .zip(ys.iter())
            .map(|(x, y)| Self::basis(x, &xs).scalar_mul(y))
            .sum()
    }

    fn scalar_mul(&self, scalar: &F) -> Self {
        UnivariatePoly {
            coefficients: self
                .coefficients
                .iter()
                .map(|coeff| *scalar * *coeff)
                .collect(),
        }
    }

    fn basis(x: &F, interpolating_set: &[F]) -> Self {
        let numerator: UnivariatePoly<F> = interpolating_set
            .iter()
            .filter(|val| *val != x)
            .map(|x_n| UnivariatePoly::new(vec![x_n.neg(), F::one()]))
            .product();

        let denominator = F::one() / numerator.evaluate(*x);

        numerator.scalar_mul(&denominator)
    }
}

impl<F: PrimeField> Mul for UnivariatePoly<F> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        // mul for dense polynomials
        let mut result = vec![F::zero(); (self.degree() + rhs.degree() + 1).try_into().unwrap()];
        for (i, coeff1) in self.coefficients.iter().enumerate() {
            for (j, coeff2) in rhs.coefficients.iter().enumerate() {
                result[i + j] += *coeff1 * *coeff2;
            }
        }
        UnivariatePoly {
            coefficients: result,
        }
    }
}

impl<F: PrimeField> Sum for UnivariatePoly<F> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(UnivariatePoly::new(vec![F::zero()]), |acc, x| acc + x)
    }
}

impl<F: PrimeField> Product for UnivariatePoly<F> {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(UnivariatePoly::new(vec![F::one()]), |acc, x| acc * x)
    }
}

impl<F: PrimeField> Add for UnivariatePoly<F> {
    type Output = UnivariatePoly<F>;

    fn add(self, rhs: Self) -> Self::Output {
        let (mut bigger, smaller) = if self.degree() < rhs.degree() {
            (rhs.clone(), self)
        } else {
            (self.clone(), rhs)
        };

        let _ = bigger
            .coefficients
            .iter_mut()
            .zip(smaller.coefficients.iter())
            .map(|(b_coeff, s_coeff)| *b_coeff += s_coeff)
            .collect::<()>();

        UnivariatePoly::new(bigger.coefficients)
    }
}

fn main() {}

#[cfg(test)]
mod test {
    use super::*;
    use ark_bn254::Fq;

    #[test]
    fn test_dense_polynomials() {
        // Test zero polynomial
        let zero_poly = UnivariatePoly::<Fq> {
            coefficients: vec![Fq::from(0)],
        };

        assert_eq!(zero_poly.degree(), 0);
        assert_eq!(zero_poly.evaluate(Fq::from(5)), Fq::from(0));
        assert_eq!(
            UnivariatePoly::interpolate(vec![Fq::from(1)], vec![Fq::from(0)]).coefficients,
            vec![Fq::from(0)]
        );

        // Test single coefficient polynomial
        let constant_poly = UnivariatePoly::<Fq> {
            coefficients: vec![Fq::from(7)],
        };
        assert_eq!(constant_poly.degree(), 0);
        assert_eq!(constant_poly.evaluate(Fq::from(10)), Fq::from(7));
        assert_eq!(
            UnivariatePoly::interpolate(vec![Fq::from(10)], vec![Fq::from(7)]).coefficients,
            vec![Fq::from(7)]
        );
    }
    #[test]
    fn test_fibonnacci_verification() {
        let fibonacci_poly = UnivariatePoly::interpolate(
            vec![
                Fq::from(0),
                Fq::from(1),
                Fq::from(2),
                Fq::from(3),
                Fq::from(4),
                Fq::from(5),
                Fq::from(6),
                Fq::from(7),
            ],
            vec![
                Fq::from(1),
                Fq::from(1),
                Fq::from(2),
                Fq::from(3),
                Fq::from(5),
                Fq::from(8),
                Fq::from(13),
                Fq::from(21),
            ],
        );

        assert_eq!(fibonacci_poly.degree(), 7);
        assert_eq!(
            fibonacci_poly.evaluate(Fq::from(7)),
            fibonacci_poly.evaluate(Fq::from(6)) + fibonacci_poly.evaluate(Fq::from(5))
        );
        assert_eq!(
            fibonacci_poly.evaluate(Fq::from(9)),
            fibonacci_poly.evaluate(Fq::from(7)) + fibonacci_poly.evaluate(Fq::from(8))
        );
        assert_eq!(fibonacci_poly.evaluate(Fq::from(1)), Fq::from(0));
        assert_eq!(fibonacci_poly.evaluate(Fq::from(8)), Fq::from(13));
    }
}
