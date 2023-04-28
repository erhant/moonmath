use ark_ec::{AffineCurve, PairingEngine, ProjectiveCurve};
use ark_ff::One;
use ark_poly::Polynomial;
use ark_serialize::*;
use ark_std::UniformRand;

use crate::circuit::Circuit;

#[derive(Debug, CanonicalSerialize, CanonicalDeserialize)]
pub struct Setup<E: PairingEngine> {
    pub tau: Vec<E::G1Affine>,           // 1, tau, tau^2
    pub inputs: Vec<E::G1Affine>,        // rho * input_polynomial_i(tau)
    pub inputs_prime: Vec<E::G1Affine>,  // alpha_inputs * rho * input_polynomial_i(tau)
    pub outputs: Vec<E::G1Affine>,       // rho * output_polynomial_i(tau)
    pub outputs_prime: Vec<E::G1Affine>, // alpha_outputs * rho * output_polynomial_i(tau)
    pub K: Vec<E::G1Affine>, // beta * (rho * input_polynomial_i(tau) + rho^2 * output_polynomial_i(tau))
    pub alpha_inputs: E::G2Affine, // alpha_inputs
    pub alpha_outputs: E::G2Affine, // alpha_outputs
    pub gamma: E::G2Affine,  // gamma
    pub beta_gamma: E::G2Affine, // beta*gamma
    pub rho_Z: E::G2Affine,  // rho*Z(tau)
}

pub fn trusted_setup<E: PairingEngine, R: rand::Rng>(
    circuit: &Circuit<E>,
    rng: &mut R,
) -> Setup<E> {
    let degree = circuit.inputs.len();
    let tau = E::Fr::rand(rng);
    let rho = E::Fr::rand(rng);
    let alpha_inputs = E::Fr::rand(rng);
    let alpha_outputs = E::Fr::rand(rng);
    let gamma = E::Fr::rand(rng);
    let beta = E::Fr::rand(rng);

    let mut tau_i = E::Fr::one();
    let mut powers_of_tau = vec![];
    for _ in 0..degree {
        powers_of_tau.push(fr_to_g1::<E>(tau_i));
        tau_i *= tau;
    }

    let inputs = circuit
        .inputs
        .iter()
        .map(|input_polynomial| fr_to_g1::<E>(rho * input_polynomial.evaluate(&tau)))
        .collect::<Vec<E::G1Affine>>();

    let outputs = circuit
        .outputs
        .iter()
        .map(|output_polynomial| fr_to_g1::<E>(rho * output_polynomial.evaluate(&tau)))
        .collect::<Vec<E::G1Affine>>();

    let K = inputs
        .iter()
        .zip(outputs.iter())
        .map(|(input, output)| (*input + *output).mul(beta).into_affine())
        .collect::<Vec<_>>();

    Setup::<E> {
        tau: powers_of_tau,
        inputs,

        inputs_prime: circuit
            .inputs
            .iter()
            .map(|input_polynomial| {
                fr_to_g1::<E>(alpha_inputs * rho * input_polynomial.evaluate(&tau))
            })
            .collect::<Vec<_>>(),
        outputs,
        outputs_prime: circuit
            .outputs
            .iter()
            .map(|output_polynomial| {
                fr_to_g1::<E>(alpha_outputs * rho * output_polynomial.evaluate(&tau))
            })
            .collect::<Vec<_>>(),
        K,
        alpha_inputs: fr_to_g2::<E>(alpha_inputs),
        alpha_outputs: fr_to_g2::<E>(alpha_outputs),
        gamma: fr_to_g2::<E>(gamma),
        beta_gamma: fr_to_g2::<E>(beta * gamma),
        rho_Z: fr_to_g2::<E>(rho * rho * circuit.Z.evaluate(&tau)),
    }
}

pub fn fr_to_g1<E: PairingEngine>(p: E::Fr) -> E::G1Affine {
    E::G1Affine::prime_subgroup_generator().mul(p).into_affine()
}
pub fn fr_to_g2<E: PairingEngine>(p: E::Fr) -> E::G2Affine {
    E::G2Affine::prime_subgroup_generator().mul(p).into_affine()
}
