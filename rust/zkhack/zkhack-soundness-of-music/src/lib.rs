#![allow(dead_code)]
#![allow(non_snake_case)]

pub mod circuit;
pub mod data;
pub mod prover;
pub mod setup;
pub mod verifier;

pub const PUZZLE_DESCRIPTION: &str = r#"It is a well-known fact that 1+1=2. Recent work by Alice von Trapp (et al) suggests that under
special conditions in the Swiss Alps, 1+1+1+1=1. Alice has been unable to prove this statement over the BLS12-381 scalar field. 
The primary difficulty appears to be the fact that 1 is not equal to 4. Alice's proving system
can write a proof for every statement (x,y) where x and y are BLS12-381 scalars, and x+x+x+x=y.
The proving system easily outputs a proof for the statement (1,4) showing 1+1+1+1=4, but seems
unable to produce a proof for the statement (1,1) showing 1+1+1+1=1.

A proof for the statement (1,1) is badly needed for the elliptic curve finale song to be added to Appendix A.
Can you save the show and output a proof where verify(&[Fr::one(), Fr::one()], &setup, &proof) = true ? "#;

#[test]
fn test_snark() {
    use ark_ff::One;

    type Fr = <ark_bls12_381::Bls12_381 as ark_ec::PairingEngine>::Fr;

    let circuit = circuit::make_circuit::<ark_bls12_381::Bls12_381>();

    let rng = &mut ark_std::test_rng();
    let setup = setup::trusted_setup(&circuit, rng);

    // Successfully prove that 1+1+1+1=4
    let two = Fr::one() + Fr::one();
    let four = two + two;
    let public_inputs = [Fr::one(), four];
    let private_inputs = [two];
    let proof = prover::prove(&public_inputs, &private_inputs, &circuit, &setup);

    assert!(verifier::verify(&public_inputs, &setup, &proof));

    // Fail to prove that 1+1+1+1=1
    let public_inputs = [Fr::one(), Fr::one()];
    let private_inputs = [two];
    let proof = prover::prove(&public_inputs, &private_inputs, &circuit, &setup);

    assert!(!verifier::verify(&public_inputs, &setup, &proof));
}
