use std::iter::{Product, Sum};
use std::ops::{Add, Mul};

#[derive(Debug, PartialEq, Clone)]
struct UnivariatePoly {
    coefficients: Vec<f64>,
}

impl UnivariatePoly {
    fn new(coefficients: Vec<f64>) -> Self {
        UnivariatePoly { coefficients }
    }

    fn degree(&self) -> i32 {
        self.coefficients.len() as i32 - 1
    }

    fn evaluate(&self, x: f64) -> f64 {
        self.coefficients
            .iter()
            .rev()
            .cloned()
            .reduce(|acc, curr| acc * x + curr)
            .unwrap()
    }

    fn interpolate(xs: Vec<f64>, ys: Vec<f64>) -> Self {
        xs.iter()
            .zip(ys.iter())
            .map(|(x, y)| Self::basis(x, &xs).scalar_mul(y))
            .sum()
    }

    fn scalar_mul(&self, scalar: &f64) -> Self {
        UnivariatePoly {
            coefficients: self
                .coefficients
                .iter()
                .map(|coeff| scalar * coeff)
                .collect(),
        }
    }

    fn basis(x: &f64, interpolating_set: &[f64]) -> Self {
        let numerator: UnivariatePoly = interpolating_set
            .iter()
            .filter(|val| *val != x)
            .map(|x_n| UnivariatePoly::new(vec![-x_n, 1.0]))
            .product();

        let denominator = 1.0 / numerator.evaluate(*x);

        numerator.scalar_mul(&denominator)
    }
}

impl Mul for UnivariatePoly {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        // mul for dense polynomials
        let mut result = vec![0.0; (self.degree() + rhs.degree() + 1).try_into().unwrap()];
        for (i, coeff1) in self.coefficients.iter().enumerate() {
            for (j, coeff2) in rhs.coefficients.iter().enumerate() {
                result[i + j] += coeff1 * coeff2;
            }
        }
        UnivariatePoly {
            coefficients: result,
        }
    }
}

impl Sum for UnivariatePoly {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(UnivariatePoly::new(vec![0.0]), |acc, x| acc + x)
    }
}

impl Product for UnivariatePoly {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(UnivariatePoly::new(vec![1.0]), |acc, x| acc * x)
    }
}

impl Add for UnivariatePoly {
    type Output = UnivariatePoly;

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

    #[test]
    fn test_dense_polynomials() {
        // Test zero polynomial
        let zero_poly = UnivariatePoly {
            coefficients: vec![0.0],
        };
        assert_eq!(zero_poly.degree(), 0);
        assert_eq!(zero_poly.evaluate(5.0), 0.0);
        assert_eq!(
            UnivariatePoly::interpolate(vec![1.0], vec![0.0]).coefficients,
            vec![0.0]
        );

        // Test single coefficient polynomial
        let constant_poly = UnivariatePoly {
            coefficients: vec![7.0],
        };
        assert_eq!(constant_poly.degree(), 0);
        assert_eq!(constant_poly.evaluate(10.0), 7.0);
        assert_eq!(
            UnivariatePoly::interpolate(vec![10.0], vec![7.0]).coefficients,
            vec![7.0]
        );
    }
}
