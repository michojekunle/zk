use ark_bn254::Fq;
use ark_ff::PrimeField;
use polynomial::UnivariatePoly;
use rand;

#[derive(Debug)]
struct Point<F> {
    x: F,
    y: F,
}

fn generate_shares<F: PrimeField>(
    secret: i32,
    password: i32,
    threshold: usize,
    total_shares: usize,
) -> Vec<Point<F>> {
    assert!(
        threshold <= total_shares,
        "Threshold must be less than or equal to total shares"
    );
    assert!(threshold > 0, "Threshold must be greater than 0");

    let mut rng = rand::thread_rng();

    let mut xs: Vec<F> = Vec::new();
    let mut ys: Vec<F> = Vec::new();

    xs.push(F::from(password));
    ys.push(F::from(secret));

    for _ in 1..threshold {
        xs.push(F::rand(&mut rng));
        ys.push(F::rand(&mut rng));
    }

    let poly = UnivariatePoly::interpolate(xs, ys);

    if poly.degree() != (threshold - 1).try_into().unwrap() {
        panic!("Failed to interpolate polynomial");
    }

    // Generate points (shares)
    let mut shares = Vec::new();
    for _ in 1..=total_shares {
        let x = F::rand(&mut rng);
        let y = poly.evaluate(x);
        shares.push(Point { x, y });
    }

    shares
}

fn reconstruct_secret<F: PrimeField>(
    shares: &[Point<F>],
    password: i32,
    threshold: usize,
) -> Option<F> {
    if shares.len() < threshold {
        return None;
    }

    // Prepare points for interpolation
    let xs: Vec<F> = shares[0..threshold].iter().map(|p| p.x).collect();
    let ys: Vec<F> = shares[0..threshold].iter().map(|p| p.y).collect();

    // Use Lagrange interpolation
    let poly = UnivariatePoly::interpolate(xs, ys);

    // The secret is the constant term (evaluate at x = password)
    Some(poly.evaluate(F::from(password)))
}

fn main() {
    generate_shares::<Fq>(500, 25, 4, 10);
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_bn254::Fq;

    #[test]
    fn test_basic_sharing_and_reconstruction() {
        let secret = 42;
        let threshold = 3;
        let total_shares = 5;
        let password = 25;

        let shares = generate_shares::<Fq>(secret, password, threshold, total_shares);
        assert_eq!(shares.len(), total_shares);

        let reconstructed = reconstruct_secret(&shares[..threshold], password, threshold);
        assert_eq!(reconstructed, Some(Fq::from(secret)));
    }

    #[test]
    fn test_reconstruction_with_different_share_combinations() {
        let secret = 123;
        let threshold = 3;
        let total_shares = 5;
        let password = 25;

        let shares = generate_shares::<Fq>(secret, password, threshold, total_shares);

        let reconstructed1 = reconstruct_secret(&shares[1..4], password, threshold);
        let reconstructed2 = reconstruct_secret(&shares[2..5], password, threshold);

        assert_eq!(reconstructed1, Some(Fq::from(secret)));
        assert_eq!(reconstructed2, Some(Fq::from(secret)));
    }

    #[test]
    fn test_insufficient_shares() {
        let secret = 42;
        let threshold = 3;
        let total_shares = 5;
        let password = 25;

        let shares = generate_shares::<Fq>(secret, password, threshold, total_shares);
        let reconstructed = reconstruct_secret(&shares[..2], password, threshold);
        assert_eq!(reconstructed, None);
    }

    #[test]
    #[should_panic]
    fn test_invalid_threshold() {
        generate_shares::<Fq>(42, 15, 0, 5);
    }
}
