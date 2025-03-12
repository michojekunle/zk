use crate::fiat_shamir::FiatShamir;
use polynomials::multilinear::multilinear_poly::MultilinearPoly;
use ark_ff::{BigInteger, PrimeField};
use sha3::Keccak256;

pub struct Proof<F: PrimeField> {
    claimed_sum: F,
    round_polys: Vec<[F; 2]>,
}

pub struct PartialProof<F: PrimeField> {
    claimed_sum: F,
    round_polys: Vec<[F; 3]>,
}

pub fn prove<F: PrimeField>(poly: &MultilinearPoly<F>, claimed_sum: F) -> Proof<F> {
    let mut round_polys: Vec<[F; 2]> = vec![];

    // public
    // poly
    // claimed_sum

    let mut transcript = FiatShamir::<Keccak256, F>::new();

    transcript.absorb(
        poly.evals
            .iter()
            .flat_map(|f| f.into_bigint().to_bytes_be())
            .collect::<Vec<_>>()
            .as_slice(),
    );

    transcript.absorb(claimed_sum.into_bigint().to_bytes_be().as_slice());

    let mut poly = poly.clone();

    for i in 0..poly.n_vars {
        let round_poly: [F; 2] = [
            poly.partial_evaluate((poly.n_vars - 1, F::zero()))
                .evals
                .iter()
                .sum(),
            poly.partial_evaluate((poly.n_vars - 1, F::one()))
                .evals
                .iter()
                .sum(),
        ];

        transcript.absorb(
            round_poly
                .iter()
                .flat_map(|f| f.into_bigint().to_bytes_be())
                .collect::<Vec<_>>()
                .as_slice(),
        );
        round_polys.push(round_poly);

        let challenge = transcript.squeeze();

        poly = poly.partial_evaluate((poly.n_vars - 1, challenge))
    }

    Proof {
        claimed_sum,
        round_polys,
    }
}

pub fn partial_prove<F: PrimeField>(
    poly: &MultilinearPoly<F>,
    claimed_sum: F,
    transcript: &mut FiatShamir<Keccak256, F>,
) -> PartialProof<F> {
    let mut round_polys: Vec<[F; 3]> = vec![];

    // public
    // poly
    // claimed_sum

    transcript.absorb(
        poly.evals
            .iter()
            .flat_map(|f| f.into_bigint().to_bytes_be())
            .collect::<Vec<_>>()
            .as_slice(),
    );

    transcript.absorb(claimed_sum.into_bigint().to_bytes_be().as_slice());

    let mut poly = poly.clone();

    for i in 0..poly.n_vars {
        let round_poly: [F; 3] = [
            poly.partial_evaluate((poly.n_vars - 1, F::zero()))
                .evals
                .iter()
                .sum(),
            poly.partial_evaluate((poly.n_vars - 1, F::one()))
                .evals
                .iter()
                .sum(),
            poly.partial_evaluate((poly.n_vars - 1, F::from(2)))
                .evals
                .iter()
                .sum(),
        ];

        transcript.absorb(
            round_poly
                .iter()
                .flat_map(|f| f.into_bigint().to_bytes_be())
                .collect::<Vec<_>>()
                .as_slice(),
        );
        round_polys.push(round_poly);

        let challenge = transcript.squeeze();

        poly = poly.partial_evaluate((poly.n_vars - 1, challenge))
    }

    PartialProof {
        claimed_sum,
        round_polys,
    }
}

pub fn verify<F: PrimeField>(proof: &Proof<F>, poly: &mut MultilinearPoly<F>) -> bool {
    if proof.round_polys.len() != poly.n_vars {
        return false;
    }

    let mut challenges = vec![];

    let mut transcript = FiatShamir::<Keccak256, F>::new();

    transcript.absorb(
        poly.evals
            .iter()
            .flat_map(|f| f.into_bigint().to_bytes_be())
            .collect::<Vec<_>>()
            .as_slice(),
    );

    transcript.absorb(proof.claimed_sum.into_bigint().to_bytes_be().as_slice());

    let mut claimed_sum = proof.claimed_sum;

    for round_poly in &proof.round_polys {
        if claimed_sum != round_poly.iter().sum() {
            return false;
        }

        transcript.absorb(
            round_poly
                .iter()
                .flat_map(|f| f.into_bigint().to_bytes_be())
                .collect::<Vec<_>>()
                .as_slice(),
        );

        let challenge = transcript.squeeze();

        challenges.push(challenge);

        claimed_sum = round_poly[0] + challenge * (round_poly[1] - round_poly[0])
    }

    if claimed_sum != poly.evaluate(challenges) {
        return false;
    }

    true
}

pub fn partial_verify<F: PrimeField>(
    proof: &PartialProof<F>,
    poly: &mut MultilinearPoly<F>,
    transcript: &mut FiatShamir<Keccak256, F>,
) -> (Vec<F>, F) {
    if proof.round_polys.len() != poly.n_vars {
        return (vec![], F::zero());
    }

    let mut challenges = vec![];

    transcript.absorb(
        poly.evals
            .iter()
            .flat_map(|f| f.into_bigint().to_bytes_be())
            .collect::<Vec<_>>()
            .as_slice(),
    );

    transcript.absorb(proof.claimed_sum.into_bigint().to_bytes_be().as_slice());

    let mut claimed_sum = proof.claimed_sum;

    for round_poly in &proof.round_polys {
        if claimed_sum != round_poly.iter().sum() {
            return (vec![], F::zero());
        }

        transcript.absorb(
            round_poly
                .iter()
                .flat_map(|f| f.into_bigint().to_bytes_be())
                .collect::<Vec<_>>()
                .as_slice(),
        );

        let challenge = transcript.squeeze();

        challenges.push(challenge);

        claimed_sum = round_poly[0] + challenge * (round_poly[1] - round_poly[0])
    }

    // if claimed_sum != poly.evaluate(challenges) {
    //     return false;
    // }

    // true
    (challenges, claimed_sum)
}

#[cfg(test)]
mod tests {
    use polynomials::multilinear::multilinear_poly::MultilinearPoly;
    use crate::sumcheck_protocol::{prove, verify};
    use ark_bn254::Fr;

    pub fn to_field(input: Vec<u64>) -> Vec<Fr> {
        input.iter().map(|v| Fr::from(*v)).collect()
    }

    #[test]
    pub fn test_sumcheck() {
        let mut poly = MultilinearPoly::new(to_field(vec![0, 0, 0, 3, 0, 0, 2, 5]), 3);
        let proof = prove(&poly, Fr::from(10));

        dbg!(verify(&proof, &mut poly));
    }

    #[test]
    pub fn test_partial_sumcheck_gkr() {}
}
