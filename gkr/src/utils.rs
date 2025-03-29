use ark_ff::PrimeField;
use polynomials::multilinear::multilinear_poly::MultilinearPoly;

pub fn get_evaluated_muli_addi_at_a<F: PrimeField>(
    muli_a_b_c: MultilinearPoly<F>,
    addi_a_b_c: MultilinearPoly<F>,
    random_values: Vec<F>
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
    todo!()
}

pub fn get_folded_claim_sum<F: PrimeField>(
    alpha: &F,
    beta: &F,
    w_i_b_eval: &F,
    w_i_c_eval: &F,
) -> F {
    (*alpha * *w_i_b_eval) + (*beta * *w_i_c_eval)
}
