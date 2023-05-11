#![allow(unused, unreachable_code)]
use ark_ed_on_bls12_381::Fr;
use ark_ff::Field;
use strong_adaptivity::{Instance, Proof, data::puzzle_data};
use strong_adaptivity::verify;
use strong_adaptivity::PUZZLE_DESCRIPTION;
use prompt::{puzzle, welcome};

fn main() {
    welcome();
    puzzle(PUZZLE_DESCRIPTION);
    let ck = puzzle_data();

    let (instance, witness, proof): (Instance, (Fr, Fr, Fr, Fr), Proof) = {
        // Your solution here!
        todo!()
    };
    
    let (a_1, r_1, a_2, r_2) = witness;

    assert!(verify(&ck, &instance, &proof));
    // Check that commitments are correct
    assert_eq!(ck.commit_with_explicit_randomness(a_1, r_1), instance.comm_1);
    assert_eq!(ck.commit_with_explicit_randomness(a_2, r_2), instance.comm_2);
    // Check that messages are unequal
    assert_ne!(a_1, a_2);
}