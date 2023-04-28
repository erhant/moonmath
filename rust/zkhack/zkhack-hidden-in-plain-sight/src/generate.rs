#![allow(dead_code, unused_imports)]

use ark_bls12_381::{Fr, G1Affine, G1Projective};
use ark_ec::{AffineCurve, ProjectiveCurve};
use ark_ff::*;
use ark_poly::{
    univariate::DensePolynomial, EvaluationDomain, GeneralEvaluationDomain, Polynomial,
    UVPolynomial,
};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};

fn generate_kzg_setup(n: usize) -> Vec<G1Affine> {
    let mut rng = rand::thread_rng();
    let secret_scalar = Fr::rand(&mut rng);
    let secret_powers: Vec<Fr> = (0..n as u64)
        .map(|p| secret_scalar.pow(&[p, 0, 0, 0]))
        .collect();
    let generator = G1Projective::prime_subgroup_generator();
    let kzg_setup: Vec<G1Affine> = secret_powers
        .iter()
        .map(|s| (generator.mul(s.into_repr())).into_affine())
        .collect();

    kzg_setup
}

pub fn kzg_commit(p: &DensePolynomial<Fr>, setup: &Vec<G1Affine>) -> G1Affine {
    p.coeffs()
        .iter()
        .zip(setup)
        .map(|(c, p)| p.into_projective().mul(c.into_repr()))
        .sum::<G1Projective>()
        .into_affine()
}

fn generate_acct() -> Vec<Fr> {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let acct_bytes: Vec<u8> = (0..32).into_iter().map(|_u| rng.gen::<u8>()).collect();
    acct_bytes.iter().map(|u| Fr::from(*u)).collect()
}

fn generate_accts(n: usize) -> Vec<Vec<Fr>> {
    (0..n)
        .into_iter()
        .map(|_u| generate_acct())
        .collect::<Vec<_>>()
}

fn generate_challenge() -> (Vec<G1Affine>, Vec<Vec<Fr>>, Fr, Fr, G1Affine, Fr, Fr) {
    use rand::Rng;
    let mut rng = rand::thread_rng();

    let number_of_accts = 1000usize;
    let accts = generate_accts(number_of_accts);
    let target_acct_index = rng.gen_range(0..number_of_accts);
    let target_acct = &accts[target_acct_index];

    let domain: GeneralEvaluationDomain<Fr> =
        GeneralEvaluationDomain::new(number_of_accts + 2).unwrap();
    let setup = generate_kzg_setup(domain.size());

    let target_acct_poly = DensePolynomial::from_coefficients_vec(domain.ifft(&target_acct));
    let blinding_poly =
        DensePolynomial::from_coefficients_vec(vec![Fr::rand(&mut rng), Fr::rand(&mut rng)]);
    let blinded_acct_poly = target_acct_poly + blinding_poly.mul_by_vanishing_poly(domain);

    let commitment: G1Affine = kzg_commit(&blinded_acct_poly, &setup);

    let challenge_1 = Fr::rand(&mut rng);
    let challenge_2 = Fr::rand(&mut rng);

    let opening_1 = blinded_acct_poly.evaluate(&challenge_1);
    let opening_2 = blinded_acct_poly.evaluate(&challenge_2);

    (
        setup,
        accts,
        challenge_1,
        challenge_2,
        commitment,
        opening_1,
        opening_2,
    )
}
