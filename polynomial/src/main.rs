// sparse univariate polynomial
#[derive(Clone, Debug)]
struct UnivariatePoly {
    degree: i32,
    coefficients: Vec<(f64, i32)>,
}

// helper function to simplify tuples for composing tuples with
// the same exponent and also simplifying adding polynomials function
fn simplify_tuples(vec: Vec<(f64, i32)>) -> Vec<(f64, i32)> {
    use std::collections::HashMap;

    let mut map: HashMap<i32, f64> = HashMap::new();

    // Group coefficients by exponent and sum them
    for (coeff, exp) in vec {
        *map.entry(exp).or_insert(0.0) += coeff;
    }

    // Convert the HashMap back into a vector of tuples, swapping (i32, f64) to (f64, i32)
    let mut simplified: Vec<(f64, i32)> = map
        .into_iter()
        .filter(|&(_, c)| c != 0.0)
        .map(|(exp, coeff)| (coeff, exp))
        .collect();

    // Sort the vector by exponent for consistency
    simplified.sort_by_key(|&(_, exp)| std::cmp::Reverse(exp));

    simplified
}

// in the form: yn . Ln(x)
fn lagrange_basis(point: (u32, u32), interpolating_poly: Vec<u32>) -> UnivariatePoly {
    let (x, y) = point;

    let mut polys: Vec<UnivariatePoly> = Vec::new();
    let mut int_prod: i32 = 1;

    for val in interpolating_poly {
        if val != x {
            polys.push(UnivariatePoly::new(vec![(1.0, 1), (-(val as f64), 0)]));
            int_prod = int_prod * (x as i32 - val as i32);
        }
    }

    println!();
    println!("Integer product: {}", int_prod);
    println!();

    let result: UnivariatePoly = polys.iter().fold(UnivariatePoly::default(), |acc, poly| {
        multiply_poly(acc, poly.clone())
    });

    // get the integer/floating point part and multiply with the resultant poly
    let non_poly_co_eff: f64 = (y as f64 / int_prod as f64).into();

    println!();
    println!("y: {} \nFloating point product: {}", y, non_poly_co_eff);
    println!();

    let mut final_vec: Vec<(f64, i32)> = Vec::new();

    for coefficient in &result.coefficients {
        let (c, e) = coefficient;

        final_vec.push((c * non_poly_co_eff, *e));
    }

    UnivariatePoly::new(final_vec)
}

fn multiply_poly(poly_a: UnivariatePoly, poly_b: UnivariatePoly) -> UnivariatePoly {
    let mut vec: Vec<(f64, i32)> = Vec::new();

    for poly_a_coefficient in poly_a.coefficients {
        let (c_a, e_a) = poly_a_coefficient;

        for poly_b_coefficient in &poly_b.coefficients {
            let (c_b, e_b) = poly_b_coefficient;
            vec.push((c_a * c_b, e_a + e_b));
        }
    }

    vec = simplify_tuples(vec);

    UnivariatePoly::new(vec)
}

fn add_poly(poly_a: UnivariatePoly, poly_b: UnivariatePoly) -> UnivariatePoly {
    let mut merged_polys: Vec<(f64, i32)> = poly_a
        .coefficients
        .iter()
        .chain(poly_b.coefficients.iter())
        .cloned()
        .collect();

    merged_polys = simplify_tuples(merged_polys);

    UnivariatePoly::new(merged_polys)
}

impl UnivariatePoly {
    fn default() -> Self {
        UnivariatePoly {
            degree: 0,
            coefficients: vec![(1.0, 0)],
        }
    }

    fn new(coefficients: Vec<(f64, i32)>) -> UnivariatePoly {
        let degree = coefficients.iter().map(|(_, d)| d).max().unwrap();
        UnivariatePoly {
            degree: *degree,
            coefficients,
        }
    }

    fn degree(&self) -> i32 {
        self.degree
    }

    fn evaluate(&self, x: i32) -> i32 {
        self.coefficients
            .iter()
            .map(|(c, d)| (c * (x.pow(*d as u32) as f64)) as i32)
            .sum()
    }

    fn interpolate(&self, eval: Vec<(u32, u32)>) -> UnivariatePoly {
        let xs: Vec<u32> = eval.iter().map(|(x, _)| *x).collect();
        let mut sum: UnivariatePoly = UnivariatePoly::new(vec![(0.0, 0)]);

        if self.degree < 1 {
            return UnivariatePoly::new(vec![(0.0, 0)]);
        }

        for point in eval {
            let curr: UnivariatePoly = lagrange_basis(point, xs.clone());
            println!();
            println!("Curr poly is: {:?}", curr.coefficients);
            println!();
            sum = add_poly(sum, curr);
        }

        sum
    }
}

// dense polynomial
struct DenseUnivariatePoly {
    coefficients: Vec<u32>,
}

impl DenseUnivariatePoly {
    fn degree(&self) -> u32 {
        (self.coefficients.len() - 1) as u32
    }

    fn evaluate(&self, x: u32) -> u32 {
        self.coefficients
            .iter()
            .enumerate()
            .map(|(i, c)| c * x.pow(i as u32))
            .sum()
    }
}

fn main() {
    // test sparse polynomial
    let poly_1 = UnivariatePoly::new(vec![(2.0, 1), (5.0, 0)]);
    let poly_1_degree = poly_1.degree();
    let poly_1_eval_2 = poly_1.evaluate(2);
    let poly_1_eval_3 = poly_1.evaluate(3);
    let poly_1_interpolated = poly_1.interpolate(vec![(2, 9), (3, 11)]);

    assert_eq!(poly_1_degree, 1);
    assert_eq!(poly_1_eval_2, 9);
    assert_eq!(poly_1_eval_3, 11);
    assert_eq!(poly_1_interpolated.coefficients, vec![(2.0, 1), (5.0, 0)]);

    println!("Sparse Poly 1 degree: {}", poly_1_degree);
    println!("Sparse Poly 1 evaluated at 2: {}", poly_1_eval_2);
    println!("Sparse Poly 1 evaluated at 3: {}", poly_1_eval_3);
    println!(
        "Sparse Poly 1 interpolated given evaluation format: {:?}",
        poly_1_interpolated.coefficients
    );
    println!();

    let poly_2 = UnivariatePoly::new(vec![(3.0, 2), (2.0, 1), (5.0, 0)]);
    let poly_2_degree = poly_2.degree();
    let poly_2_eval_2 = poly_2.evaluate(2);
    let poly_2_eval_3 = poly_2.evaluate(3);
    let poly_2_interpolated = poly_2.interpolate(vec![(2, 21), (3, 38), (1, 10)]);

    assert_eq!(poly_2_degree, 2);
    assert_eq!(poly_2_eval_2, 21);
    assert_eq!(poly_2_eval_3, 38);
    assert_eq!(
        poly_2_interpolated.coefficients,
        vec![(3.0, 2), (2.0, 1), (5.0, 0)]
    );

    println!("Sparse Poly 2 degree: {}", poly_2_degree);
    println!("Sparse Poly 2 evaluated at 2: {}", poly_2_eval_2);
    println!("Sparse Poly 2 evaluated at 3: {}", poly_2_eval_3);
    println!(
        "Sparse Poly 2 interpolated given evaluation format: {:?}",
        poly_2_interpolated.coefficients
    );
    println!();

    // test dense polynomial
    let dense_poly_1 = DenseUnivariatePoly {
        coefficients: vec![5, 2],
    };

    let dense_poly_1_degree = dense_poly_1.degree();
    let dense_poly_1_eval_2 = dense_poly_1.evaluate(2);
    let dense_poly_1_eval_3 = dense_poly_1.evaluate(3);

    assert_eq!(dense_poly_1_degree, 1);
    assert_eq!(dense_poly_1_eval_2, 9);
    assert_eq!(dense_poly_1_eval_3, 11);

    println!("Dense Poly 1 degree: {}", dense_poly_1_degree);
    println!("Dense Poly 1 evaluated at 2: {}", dense_poly_1_eval_2);
    println!("Dense Poly 1 evaluated at 3: {}", dense_poly_1_eval_3);
    println!();

    let dense_poly_2 = DenseUnivariatePoly {
        coefficients: vec![5, 2, 3],
    };

    let dense_poly_2_degree = dense_poly_2.degree();
    let dense_poly_2_eval_2 = dense_poly_2.evaluate(2);
    let dense_poly_2_eval_3 = dense_poly_2.evaluate(3);

    assert_eq!(dense_poly_2_degree, 2);
    assert_eq!(dense_poly_2_eval_2, 21);
    assert_eq!(dense_poly_2_eval_3, 38);

    println!("Dense Poly 2 degree: {}", dense_poly_2_degree);
    println!("Dense Poly 2 evaluated at 2: {}", dense_poly_2_eval_2);
    println!("Dense Poly 2 evaluated at 3: {}", dense_poly_2_eval_3);
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sparse_polynomials() {
        let poly = UnivariatePoly::new(vec![(1.0, 3), (2.0, 2), (3.0, 1), (4.0, 0)]);
        assert_eq!(poly.degree(), 3);
        assert_eq!(poly.evaluate(1), 10);
        assert_eq!(poly.evaluate(2), 26);
        assert_eq!(poly.evaluate(3), 58);
        assert_eq!(poly.evaluate(4), 112);
        // Test interpolation with different points
        assert_eq!(
            poly.interpolate(vec![(2, 26), (3, 58), (1, 10), (4, 112)])
                .coefficients,
            vec![(1.0, 3), (2.0, 2), (3.0, 1), (4.0, 0)]
        );

        // Test zero polynomial interpolation
        let zero_poly = UnivariatePoly::new(vec![(0.0, 0)]);
        assert_eq!(zero_poly.degree(), 0);
        assert_eq!(zero_poly.evaluate(5), 0);
        assert_eq!(
            zero_poly
                .interpolate(vec![(1, 0), (2, 0), (3, 0)])
                .coefficients,
            vec![(0.0, 0)]
        );

        // Test single term polynomial
        let single_term = UnivariatePoly::new(vec![(3.0, 4)]);
        assert_eq!(single_term.degree(), 4);
        assert_eq!(single_term.evaluate(2), 48);
        assert_eq!(
            single_term
                .interpolate(vec![(1, 3), (2, 48), (3, 243), (4, 768), (5, 1875)])
                .coefficients,
            vec![(3.0, 4)]
        );
    }

    #[test]
    fn test_dense_polynomials() {
        let poly = DenseUnivariatePoly {
            coefficients: vec![4, 3, 2, 1],
        };
        assert_eq!(poly.degree(), 3);
        assert_eq!(poly.evaluate(2), 26);
        assert_eq!(poly.evaluate(3), 58);

        // Test zero polynomial
        let zero_poly = DenseUnivariatePoly {
            coefficients: vec![0],
        };
        assert_eq!(zero_poly.degree(), 0);
        assert_eq!(zero_poly.evaluate(5), 0);

        // Test single coefficient polynomial
        let constant_poly = DenseUnivariatePoly {
            coefficients: vec![7],
        };
        assert_eq!(constant_poly.degree(), 0);
        assert_eq!(constant_poly.evaluate(10), 7);
    }
}
