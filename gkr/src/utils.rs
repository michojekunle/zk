use ark_ff::PrimeField;
use polynomials::multilinear::multilinear_poly::MultilinearPoly;

pub fn get_evaluated_muli_addi_at_a<F: PrimeField>(
    muli_a_b_c: MultilinearPoly<F>,
    addi_a_b_c: MultilinearPoly<F>,
    random_values: Vec<F>,
) -> (MultilinearPoly<F>, MultilinearPoly<F>) {
    let mut new_muli_b_c = muli_a_b_c;
    let mut new_addi_b_c = addi_a_b_c;

    for i in 0..random_values.len() {
        new_muli_b_c = new_muli_b_c.partial_evaluate((new_muli_b_c.n_vars - 1, random_values[i]));
        new_addi_b_c = new_addi_b_c.partial_evaluate((new_addi_b_c.n_vars - 1, random_values[i]));
    }

    (new_muli_b_c, new_addi_b_c)
}

pub fn get_folded_polys<F: PrimeField>(
    alpha: &F,
    beta: &F,
    muli_a_b_c: MultilinearPoly<F>,
    addi_a_b_c: MultilinearPoly<F>,
    r_b: &[F],
    r_c: &[F],
) -> (MultilinearPoly<F>, MultilinearPoly<F>) {
    // Apply partial evaluation for r_b and scale by alpha
    let muli_b = r_b
        .iter()
        .fold(muli_a_b_c.clone(), |mut acc, &b| {
            acc.partial_evaluate((acc.n_vars - 1, b))
        })
        .scalar_mul(*alpha);
    let addi_b = r_b
        .iter()
        .fold(addi_a_b_c.clone(), |mut acc, &b| {
            acc.partial_evaluate((acc.n_vars - 1, b))
        })
        .scalar_mul(*alpha);

    // Apply partial evaluation for r_c and scale by beta
    let muli_c = r_c
        .iter()
        .fold(muli_a_b_c, |mut acc, &c| {
            acc.partial_evaluate((acc.n_vars - 1, c))
        })
        .scalar_mul(*beta);
    let addi_c = r_c
        .iter()
        .fold(addi_a_b_c, |mut acc, &c| {
            acc.partial_evaluate((acc.n_vars - 1, c))
        })
        .scalar_mul(*beta);

    // Sum the results
    (muli_b + muli_c, addi_b + addi_c)
}

pub fn get_folded_claim_sum<F: PrimeField>(
    alpha: &F,
    beta: &F,
    w_i_b_eval: &F,
    w_i_c_eval: &F,
) -> F {
    (*alpha * *w_i_b_eval) + (*beta * *w_i_c_eval)
}
