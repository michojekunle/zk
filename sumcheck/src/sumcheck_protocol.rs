use crate::fiat_shamir::FiatShamir;
use ark_ff::{BigInteger, PrimeField};
use polynomials::{
    composed::sum_poly::SumPoly, multilinear::multilinear_poly::MultilinearPoly,
    univariate::univariate_poly::UnivariatePoly,
};
use sha3::Keccak256;

pub struct Proof<F: PrimeField> {
    claimed_sum: F,
    round_polys: Vec<[F; 2]>,
}

#[derive(Clone, Debug)]
pub struct PartialProof<F: PrimeField> {
    pub initial_claimed_sum: F,
    pub round_polys: Vec<UnivariatePoly<F>>,
    pub rand_challenges: Vec<F>,
}

pub fn prove<F: PrimeField>(poly: &MultilinearPoly<F>, claimed_sum: F) -> Proof<F> {
    let mut round_polys: Vec<[F; 2]> = vec![];

    let mut transcript = FiatShamir::<Keccak256, F>::new();

    transcript.absorb(
        poly.evals
            .iter()
            .flat_map(|f| f.into_bigint().to_bytes_le())
            .collect::<Vec<_>>()
            .as_slice(),
    );

    transcript.absorb(claimed_sum.into_bigint().to_bytes_le().as_slice());

    let mut poly = poly.clone();

    for i in 0..poly.n_vars {
        let idx = poly.n_vars - 1;

        let round_poly: [F; 2] = [
            poly.partial_evaluate((idx, F::zero())).evals.iter().sum(),
            poly.partial_evaluate((idx, F::one())).evals.iter().sum(),
        ];

        transcript.absorb(
            round_poly
                .iter()
                .flat_map(|f| f.into_bigint().to_bytes_le())
                .collect::<Vec<_>>()
                .as_slice(),
        );
        round_polys.push(round_poly);

        let challenge = transcript.squeeze();

        poly = poly.partial_evaluate((idx, challenge))
    }

    Proof {
        claimed_sum,
        round_polys,
    }
}

pub fn partial_prove<F: PrimeField>(
    poly: &SumPoly<F>,
    initial_claimed_sum: F,
    transcript: &mut FiatShamir<Keccak256, F>,
) -> PartialProof<F> {
    let n_vars = poly.n_vars();
    let mut round_polys: Vec<UnivariatePoly<F>> = Vec::with_capacity(n_vars.try_into().unwrap());
    let mut rand_challenges: Vec<F> = Vec::with_capacity(n_vars.try_into().unwrap());

    // transcript.absorb(&poly.to_bytes());
    // transcript.absorb(&initial_claimed_sum.into_bigint().to_bytes_le());

    let mut poly = poly.clone();

    // dbg!(&n_vars);

    for _ in 0..n_vars {
        let idx: usize = (poly.n_vars() - 1).try_into().unwrap();
        let mut claimed_sum = F::zero();

        // implement reduce func. for polynomials functions sum_poly et product_poly
        let round_poly: UnivariatePoly<F> = UnivariatePoly::interpolate(
            vec![F::zero(), F::one(), F::from(2)],
            vec![
                poly.clone()
                    .partial_evaluate((idx, F::zero()))
                    .reduce()
                    .iter()
                    .sum(),
                poly.clone()
                    .partial_evaluate((idx, F::one()))
                    .reduce()
                    .iter()
                    .sum(),
                poly.clone()
                    .partial_evaluate((idx, F::from(2)))
                    .reduce()
                    .iter()
                    .sum(),
            ],
        );

        dbg!(&round_poly);

        claimed_sum = round_poly.evaluate_sum_over_boolean_hypercube();

        // committing the claimed_sum and round_poly to the transcript
        transcript.absorb_n(&[
            &claimed_sum.into_bigint().to_bytes_le(),
            &round_poly.to_bytes(),
        ]);

        // transcript.absorb(&round_poly.to_bytes());

        round_polys.push(round_poly);

        let challenge = transcript.squeeze();

        rand_challenges.push(challenge.clone());

        dbg!(&poly.n_vars());
        dbg!(&idx);
        dbg!(&challenge);
        poly = poly.partial_evaluate((idx, challenge))
    }

    PartialProof {
        initial_claimed_sum,
        round_polys,
        rand_challenges,
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
            .flat_map(|f| f.into_bigint().to_bytes_le())
            .collect::<Vec<_>>()
            .as_slice(),
    );

    transcript.absorb(proof.claimed_sum.into_bigint().to_bytes_le().as_slice());

    let mut claimed_sum = proof.claimed_sum;

    for round_poly in &proof.round_polys {
        if claimed_sum != round_poly.iter().sum() {
            return false;
        }

        transcript.absorb(
            round_poly
                .iter()
                .flat_map(|f| f.into_bigint().to_bytes_le())
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
    transcript: &mut FiatShamir<Keccak256, F>,
) -> (Vec<F>, F) {
    let mut challenges = vec![];
    let mut claimed_sum = proof.initial_claimed_sum;

    dbg!(&proof.round_polys);

    for round_poly in &proof.round_polys {
        dbg!(&round_poly.evaluate_sum_over_boolean_hypercube());
        dbg!(&claimed_sum);

        if  round_poly.evaluate_sum_over_boolean_hypercube() != claimed_sum {
            println!("Exiting from loop hereeeeee");
            return (challenges, claimed_sum);
        }

        transcript.absorb_n(&[
            &claimed_sum.into_bigint().to_bytes_le(),
            &round_poly.to_bytes(),
        ]);

        // transcript.absorb(&round_poly.to_bytes());

        let challenge = transcript.squeeze();

        challenges.push(challenge);

        claimed_sum = round_poly.evaluate(challenge);
    }

    // if claimed_sum != poly.evaluate(challenges) {
    //     return false;
    // }

    // true
    (challenges, claimed_sum)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sumcheck_protocol::{prove, verify, partial_prove, partial_verify};
    use ark_bn254::Fr;
    use field_tracker::{print_summary, Ft};
    use polynomials::{
        composed::{product_poly::ProductPoly, sum_poly::SumPoly},
        multilinear::multilinear_poly::MultilinearPoly,
    };

    // type Fr = Ft!(ark_bn254::Fr);

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
    pub fn test_partial_sumcheck_gkr() {
        let mut transcript = FiatShamir::<Keccak256, Fr>::new();
        let (eval_1, eval_2) = (
            vec![Fr::from(0), Fr::from(0), Fr::from(0), Fr::from(2)],
            vec![Fr::from(0), Fr::from(0), Fr::from(0), Fr::from(3)],
        );

        let initial_polynomial: SumPoly<Fr> = SumPoly::new(vec![
            ProductPoly::new(vec![
                MultilinearPoly::new(eval_1.clone(), 2),
                MultilinearPoly::new(eval_2.clone(), 2),
            ]),
            ProductPoly::new(vec![
                MultilinearPoly::new(eval_1, 2),
                MultilinearPoly::new(eval_2, 2),
            ]),
        ]);

        let sum_check_proof = partial_prove(
            &initial_polynomial,
            Fr::from(12),
            &mut transcript,
        );

        let res = partial_verify(&sum_check_proof, &mut transcript);
        dbg!(&res);
        // assert!(.0);

    }

    pub fn get_2_20_poly() -> MultilinearPoly<Fr> {
        let no_of_variables = 20;
        let no_of_evals = 1 << no_of_variables;
        let mut poly_vec = Vec::with_capacity(no_of_evals);

        for i in 0..no_of_evals {
            poly_vec.push(Fr::from(i as i64));
        }

        MultilinearPoly::new(poly_vec, no_of_variables)
    }

    // #[test]
    pub fn test_sumcheck_using_field_tracker() {
        let mut poly = get_2_20_poly();
        let proof = prove(&poly, Fr::from(10));

        dbg!(verify(&proof, &mut poly));
        print_summary!();
    }
}
