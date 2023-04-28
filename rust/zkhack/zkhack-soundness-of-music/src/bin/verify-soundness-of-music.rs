#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(unreachable_code)]
#![allow(unused_imports)]

use ark_ff::One;
use prompt::{puzzle, welcome};
use soundness_of_music::data::puzzle_data;
use soundness_of_music::prover;
use soundness_of_music::verifier;
use soundness_of_music::PUZZLE_DESCRIPTION;

type Fr = <ark_bls12_381::Bls12_381 as ark_ec::PairingEngine>::Fr;

fn main() {
    welcome();
    puzzle(PUZZLE_DESCRIPTION);
    let (circuit, setup) = puzzle_data();

    let public_inputs = [Fr::one(), Fr::one()];

    /* Your solution here! */

    // Replace unimplmented!() with your proof
    assert!(verifier::verify(&public_inputs, &setup, unimplemented!()));
}
