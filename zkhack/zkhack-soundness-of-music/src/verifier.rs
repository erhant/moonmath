use crate::prover::Proof;
use crate::setup::Setup;
use ark_ec::{AffineCurve, PairingEngine, ProjectiveCurve};
use ark_ff::One;
use ark_ff::Zero;
use itertools::izip;

pub fn verify<E: PairingEngine>(
    public_inputs: &[E::Fr],
    setup: &Setup<E>,
    proof: &Proof<E>,
) -> bool {
    let mut pk = E::G1Projective::zero();

    for (public_input, public_input_polynomial) in izip!(public_inputs.iter(), setup.inputs.iter(),)
    {
        pk += public_input_polynomial.mul(*public_input);
    }
    let pk = pk.into_affine();

    let a = E::pairing(
        proof.pi_input_prime,
        E::G2Affine::prime_subgroup_generator(),
    ) == E::pairing(proof.pi_input, setup.alpha_inputs);

    let b = E::pairing(
        proof.pi_output_prime,
        E::G2Affine::prime_subgroup_generator(),
    ) == E::pairing(proof.pi_output, setup.alpha_outputs);
    let c = E::pairing(proof.pi_K, setup.gamma)
        == E::pairing(pk + proof.pi_input + proof.pi_output, setup.beta_gamma);
    let d = E::pairing(
        (pk + proof.pi_input) + (pk + proof.pi_input) + proof.pi_output.mul(-E::Fr::one()).into(),
        E::G2Affine::prime_subgroup_generator(),
    ) == E::pairing(proof.pi_H, setup.rho_Z);
    a && b && c && d
}
