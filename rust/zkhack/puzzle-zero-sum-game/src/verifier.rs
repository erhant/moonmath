use ark_ff::{to_bytes, FftField};
use ark_poly::{univariate::DensePolynomial, EvaluationDomain};
use ark_poly_commit::{Evaluations, LabeledCommitment, PolynomialCommitment, QuerySet};
use ark_std::rand::RngCore;

use crate::{
    data_structures::{Proof, Statement},
    error::Error,
    rng::FiatShamirRng,
    PROTOCOL_NAME,
};

pub fn verify<
    F: FftField,
    PC: PolynomialCommitment<F, DensePolynomial<F>>,
    FS: FiatShamirRng,
    R: RngCore,
>(
    vk: &PC::VerifierKey,
    statement: &Statement<F, PC>,
    proof: &Proof<F, PC>,
    rng: &mut R,
) -> Result<(), Error<PC::Error>> {
    let mut fs_rng = FS::initialize(&to_bytes![&PROTOCOL_NAME, statement].unwrap());

    fs_rng.absorb(&to_bytes![proof.s, proof.h, proof.g].unwrap());
    let f = LabeledCommitment::new("f".into(), statement.f.clone(), None);
    let s = LabeledCommitment::new("s".into(), proof.s.clone(), None);
    let h = LabeledCommitment::new("h".into(), proof.h.clone(), None);
    let g = LabeledCommitment::new(
        "g".into(),
        proof.g.clone(),
        Some(statement.domain.size() - 2),
    );

    let xi = F::rand(&mut fs_rng);
    let opening_challenge = F::rand(&mut fs_rng);

    let point_label = String::from("xi");
    let query_set = QuerySet::from([
        ("f".into(), (point_label.clone(), xi)),
        ("h".into(), (point_label.clone(), xi)),
        ("g".into(), (point_label.clone(), xi)),
        ("s".into(), (point_label, xi)),
    ]);

    let evaluations = Evaluations::from([
        (("f".into(), xi), proof.f_opening),
        (("s".into(), xi), proof.s_opening),
        (("h".into(), xi), proof.h_opening),
        (("g".into(), xi), proof.g_opening),
    ]);

    let res = PC::batch_check(
        vk,
        &[f, s, h, g],
        &query_set,
        &evaluations,
        &proof.pc_proof,
        opening_challenge,
        rng,
    )
    .map_err(Error::from_pc_err)?;

    if !res {
        return Err(Error::OpeningError);
    }

    let card_inverse = statement.domain.size_as_field_element().inverse().unwrap();
    let lhs = proof.s_opening + proof.f_opening;
    let rhs = {
        let x_gx = xi * proof.g_opening;
        let zh_eval = statement.domain.evaluate_vanishing_polynomial(xi);

        x_gx + proof.h_opening * zh_eval + statement.sum * card_inverse
    };

    if lhs != rhs {
        return Err(Error::IncorrectSum);
    }

    Ok(())
}
