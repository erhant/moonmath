use ark_ff::FftField;
use ark_poly::univariate::DensePolynomial;
use ark_poly_commit::{LabeledPolynomial, PolynomialCommitment};
use ark_std::rand::RngCore;

use crate::{
    data_structures::{Proof, Statement},
    error::Error,
    rng::FiatShamirRng,
};

pub fn prove<
    F: FftField,
    PC: PolynomialCommitment<F, DensePolynomial<F>>,
    FS: FiatShamirRng,
    R: RngCore,
>(
    ck: &PC::CommitterKey,
    statement: &Statement<F, PC>,
    f: &LabeledPolynomial<F, DensePolynomial<F>>,
    f_rand: &PC::Randomness,
    rng: &mut R,
) -> Result<Proof<F, PC>, Error<PC::Error>> {
    /*
        ADD YOUR CODE HERE
    */
    /*
    In the rest of protocol that is not described here, the masking polynomial is opened twice. Therefore, the masking polynomial cannot be a constant polynomial.
    */
    todo!();
}
