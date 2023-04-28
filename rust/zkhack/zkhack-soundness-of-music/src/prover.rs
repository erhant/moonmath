use crate::circuit::Circuit;
use crate::setup::Setup;
use ark_ec::{AffineCurve, PairingEngine};
use ark_ff::Zero;
use ark_poly::polynomial::univariate::DensePolynomial;
use ark_poly::UVPolynomial;
use itertools::izip;
use std::ops::Mul;

#[derive(Debug)]
pub struct Proof<E: PairingEngine> {
    pub pi_input: E::G1Affine,
    pub pi_input_prime: E::G1Affine,
    pub pi_output: E::G1Affine,
    pub pi_output_prime: E::G1Affine,
    pub pi_K: E::G1Affine,
    pub pi_H: E::G1Affine,
}

pub fn prove<E: PairingEngine>(
    public_inputs: &[E::Fr],
    private_inputs: &[E::Fr],
    circuit: &Circuit<E>,
    setup: &Setup<E>,
) -> Proof<E> {
    let private_input_start = public_inputs.len();

    let mut P = DensePolynomial::<E::Fr>::zero();
    // P = (sum_i input_i * input_polynomial) + (sum_i input_i * input_polynomial) - (sum_i input_i * output_polynomial)
    {
        for (input, input_polynomial) in izip!(
            public_inputs.iter().chain(private_inputs.iter()),
            circuit.inputs.iter(),
        ) {
            P = P + input_polynomial.mul(*input);
        }

        P = &P + &P;
        for (input, output_polynomial) in izip!(
            public_inputs.iter().chain(private_inputs.iter()),
            circuit.inputs.iter(),
        ) {
            P = P + output_polynomial.mul(-*input);
        }
    }
    // P = H*Z if and only if, on every element c of the domain, P(c) = 0
    // P(c) = 0 if and only if constraint C is satisfied by the inputs
    let H = &P / &circuit.Z;

    // pi_H is a commitment to P/Z
    let mut pi_H = E::G1Projective::zero();
    for (coeff, tau) in izip!(H.coeffs().iter(), setup.tau.iter()) {
        pi_H += tau.mul(*coeff);
    }

    let mut output_polynomials = E::G1Projective::zero();
    let mut output_polynomials_prime = E::G1Projective::zero();
    let mut private_input_polynomials = E::G1Projective::zero();
    let mut private_input_polynomials_prime = E::G1Projective::zero();
    let mut K_polynomials = E::G1Projective::zero();

    // Multiply inputs times output_polynomials, with proof of knowledge
    // output_polynomial_i = input_i * [rho*output_polynomial_i(tau)]
    // output_polynomial_prime_i = input_i * [alpha_outputs * rho*output_polynomial_i(tau)]

    for (input, public_output_polynomial, public_output_polynomial_prime) in izip!(
        public_inputs.iter().chain(private_inputs.iter()),
        setup.outputs.iter(),
        setup.outputs_prime.iter(),
    ) {
        output_polynomials += public_output_polynomial.mul(*input);
        output_polynomials_prime += public_output_polynomial_prime.mul(*input);
    }

    // Multiply inputs times K_polynomials
    // K_polynomial_i = input_i * [beta * (rho * input_polynomial_i(tau) + rho^2 * output_polynomial_i(tau))]

    for (input, public_K_polynomial) in izip!(
        public_inputs.iter().chain(private_inputs.iter()),
        setup.K.iter(),
    ) {
        K_polynomials += public_K_polynomial.mul(*input);
    }

    // Multiply private inputs times input_polynomials, with proof of knowledge
    // input_polynomial_i = input_i * [rho*input_polynomial_i(tau)]
    // input_polynomial_prime_i = input_i * [alpha_inputs * rho*input_polynomial_i(tau)]

    for (private_input, private_input_polynomial, private_input_polynomial_prime) in izip!(
        private_inputs.iter(),
        setup.inputs.iter().skip(private_input_start),
        setup.inputs_prime.iter().skip(private_input_start),
    ) {
        private_input_polynomials += private_input_polynomial.mul(*private_input);
        private_input_polynomials_prime += private_input_polynomial_prime.mul(*private_input);
    }

    Proof::<E> {
        pi_input: private_input_polynomials.into(),
        pi_input_prime: private_input_polynomials_prime.into(),
        pi_output: output_polynomials.into(),
        pi_output_prime: output_polynomials_prime.into(),
        pi_K: K_polynomials.into(),
        pi_H: pi_H.into(),
    }
}
