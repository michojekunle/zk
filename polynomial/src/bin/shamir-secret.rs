use rand::Rng;
use std::collections::HashMap;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn generate_shares(secret: i32, threshold: usize, total_shares: usize) -> Vec<Point> {
    assert!(threshold <= total_shares, "Threshold must be less than or equal to total shares");
    assert!(threshold > 0, "Threshold must be greater than 0");

    let mut rng = rand::thread_rng();
    let mut coefficients = vec![secret];
    
    // Generate random coefficients for the polynomial
    for _ in 1..threshold {
        coefficients.push(rng.gen_range(-100..100));
    }

    // Generate points (shares)
    let mut shares = Vec::new();
    for x in 1..=total_shares as i32 {
        let mut y = 0;
        for (power, coeff) in coefficients.iter().enumerate() {
            y += coeff * x.pow(power as u32);
        }
        shares.push(Point { x, y });
    }

    shares
}

fn reconstruct_secret(shares: &[Point], threshold: usize) -> Option<i32> {
    if shares.len() < threshold {
        return None;
    }

    let mut secret = 0;
    for i in 0..threshold {
        let mut numerator = 1;
        let mut denominator = 1;

        for j in 0..threshold {
            if i != j {
                numerator *= -shares[j].x;
                denominator *= shares[i].x - shares[j].x;
            }
        }

        secret += shares[i].y * numerator / denominator;
    }

    Some(secret)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_sharing() {
        let secret = 42;
        let threshold = 3;
        let total_shares = 5;

        let shares = generate_shares(secret, threshold, total_shares);
        assert_eq!(shares.len(), total_shares);

        // Test reconstruction with exact threshold number of shares
        let reconstructed = reconstruct_secret(&shares[0..threshold], threshold);
        assert_eq!(reconstructed, Some(secret));
    }

    #[test]
    fn test_reconstruction_with_more_shares() {
        let secret = 123;
        let threshold = 2;
        let total_shares = 4;

        let shares = generate_shares(secret, threshold, total_shares);
        
        // Test reconstruction with more than threshold shares
        let reconstructed = reconstruct_secret(&shares, threshold);
        assert_eq!(reconstructed, Some(secret));
    }

    #[test]
    fn test_insufficient_shares() {
        let secret = 777;
        let threshold = 3;
        let total_shares = 5;

        let shares = generate_shares(secret, threshold, total_shares);
        
        // Test reconstruction with insufficient shares
        let reconstructed = reconstruct_secret(&shares[0..threshold-1], threshold);
        assert_eq!(reconstructed, None);
    }

    #[test]
    #[should_panic]
    fn test_invalid_threshold() {
        let secret = 42;
        let threshold = 0;
        let total_shares = 5;

        generate_shares(secret, threshold, total_shares);
    }
}