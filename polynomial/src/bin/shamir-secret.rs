use ark_bn254::Fq;
use polynomial::UnivariatePoly;
use rand::Rng;

#[derive(Debug)]
struct Point {
    x: Fq,
    y: Fq,
}

fn generate_shares(secret: i32, threshold: usize, total_shares: usize) -> Vec<Point> {
    assert!(
        threshold <= total_shares,
        "Threshold must be less than or equal to total shares"
    );
    assert!(threshold > 0, "Threshold must be greater than 0");

    let mut rng = rand::thread_rng();
    let mut coefficients = vec![Fq::from(secret)];

    // Generate random coefficients for the polynomial
    for _ in 1..threshold {
        coefficients.push(Fq::from(rng.gen_range(-100..100)));
    }

    // Create polynomial using UnivariatePoly
    let poly = UnivariatePoly::new(coefficients);

    // Generate points (shares)
    let mut shares = Vec::new();
    for i in 1..=total_shares {
        let x = Fq::from(i as i32);
        let y = poly.evaluate(x);
        shares.push(Point { x, y });
    }

    dbg!(&shares);

    shares
}

fn reconstruct_secret(shares: &[Point], threshold: usize) -> Option<Fq> {
    if shares.len() < threshold {
        return None;
    }

    // Prepare points for interpolation
    let xs: Vec<Fq> = shares[0..threshold].iter().map(|p| p.x).collect();
    let ys: Vec<Fq> = shares[0..threshold].iter().map(|p| p.y).collect();

    // Use Lagrange interpolation
    let poly = UnivariatePoly::interpolate(xs, ys);

    // The secret is the constant term (evaluate at x = 0)
    Some(poly.evaluate(Fq::from(0)))
}

fn main() {
    generate_shares(500, 3, 5);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_sharing_and_reconstruction() {
        let secret = 42;
        let threshold = 3;
        let total_shares = 5;

        let shares = generate_shares(secret, threshold, total_shares);
        assert_eq!(shares.len(), total_shares);

        let reconstructed = reconstruct_secret(&shares[..threshold], threshold);
        assert_eq!(reconstructed, Some(Fq::from(secret)));
    }

    #[test]
    fn test_reconstruction_with_different_share_combinations() {
        let secret = 123;
        let threshold = 3;
        let total_shares = 5;

        let shares = generate_shares(secret, threshold, total_shares);

        let reconstructed1 = reconstruct_secret(&shares[1..4], threshold);
        let reconstructed2 = reconstruct_secret(&shares[2..5], threshold);

        assert_eq!(reconstructed1, Some(Fq::from(secret)));
        assert_eq!(reconstructed2, Some(Fq::from(secret)));
    }

    #[test]
    fn test_insufficient_shares() {
        let secret = 42;
        let threshold = 3;
        let total_shares = 5;

        let shares = generate_shares(secret, threshold, total_shares);
        let reconstructed = reconstruct_secret(&shares[..2], threshold);
        assert_eq!(reconstructed, None);
    }

    #[test]
    #[should_panic]
    fn test_invalid_threshold() {
        generate_shares(42, 0, 5);
    }
}
