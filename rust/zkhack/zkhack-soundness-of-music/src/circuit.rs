#![allow(dead_code)]
#![allow(non_snake_case)]

use ark_ec::PairingEngine;
use ark_ff::One;
use ark_ff::Zero;
use ark_poly::{
    polynomial::univariate::DensePolynomial, EvaluationDomain, Evaluations, GeneralEvaluationDomain,
};

#[derive(Debug)]
pub struct Circuit<E: PairingEngine> {
    pub inputs: Vec<DensePolynomial<E::Fr>>,  // Input wires
    pub outputs: Vec<DensePolynomial<E::Fr>>, // Output wires
    pub Z: DensePolynomial<E::Fr>,            // Variables
}

pub fn make_circuit<E: PairingEngine>() -> Circuit<E> {
    // Public inputs x, y
    // Intermediate gate output z

    // Each constraint C corresponds to a field element c in the domain
    let domain = GeneralEvaluationDomain::<E::Fr>::new(2).unwrap();

    let mut circuit = Circuit::<E> {
        inputs: vec![],  // Vector of polynomials; each corresponding to a variable
        outputs: vec![], // Vector of polynomials; each corresponding to a variable
        Z: domain.vanishing_polynomial().into(), // The zero polynomial on the domain
    };
    // A constraint C takes one input_variable and one output_variable
    // A constraint enforces:
    // input variable + input_variable => output_variable

    // First constraint: x + x => z
    // [1, 0, 0]*[x, y, z]^T + [1, 0, 0]*[x, y, z]^T  = [0, 0, 1]*[x, y, z]^T

    // Second constraint: z + z => y
    // [0, 0, 1]*[x, y, z]^T + [0, 0, 1]*[x, y, z]^T = [0, 1, 0]*[x, y, z]^T

    // First variable x
    // input_polynomial(c) evaluates to 1 when x is the input_variable to constraint C
    let input_polynomial =
        Evaluations::<E::Fr>::from_vec_and_domain(vec![E::Fr::one(), E::Fr::zero()], domain)
            .interpolate();

    // output_polynomial(c) evaluates to 1 when x is the output_variable of constraint C
    let output_polynomial =
        Evaluations::<E::Fr>::from_vec_and_domain(vec![E::Fr::zero(), E::Fr::zero()], domain)
            .interpolate();

    circuit.inputs.push(input_polynomial);
    circuit.outputs.push(output_polynomial);

    // Second variable y
    // input_polynomial(c) evaluates to 1 when y is the input_variable to constraint C

    let input_polynomial =
        Evaluations::<E::Fr>::from_vec_and_domain(vec![E::Fr::zero(), E::Fr::zero()], domain)
            .interpolate();

    // output_polynomial(c) evaluates to 1 when y is the output_variable of constraint C
    let output_polynomial =
        Evaluations::<E::Fr>::from_vec_and_domain(vec![E::Fr::zero(), E::Fr::one()], domain)
            .interpolate();

    circuit.inputs.push(input_polynomial);
    circuit.outputs.push(output_polynomial);

    // Third variable z
    // input_polynomial(c) evaluates to 1 when z is the input_variable to constraint C

    let input_polynomial =
        Evaluations::<E::Fr>::from_vec_and_domain(vec![E::Fr::zero(), E::Fr::one()], domain)
            .interpolate();

    // output_polynomial(c) evaluates to 1 when z is the output_variable of constraint C
    let output_polynomial =
        Evaluations::<E::Fr>::from_vec_and_domain(vec![E::Fr::one(), E::Fr::zero()], domain)
            .interpolate();

    circuit.inputs.push(input_polynomial);
    circuit.outputs.push(output_polynomial);
    circuit
}
