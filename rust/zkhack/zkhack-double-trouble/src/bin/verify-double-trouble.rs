#![allow(unused, unreachable_code)]
use ark_ed_on_bls12_381::Fr;
use ark_ff::Field;
use double_trouble::data::puzzle_data;
use double_trouble::inner_product_argument::utils::challenge;
use double_trouble::verify;
use double_trouble::PUZZLE_DESCRIPTION;
use prompt::{puzzle, welcome};

fn main() {
    welcome();
    puzzle(PUZZLE_DESCRIPTION);
    let (ck, [instance_and_proof_1, instance_and_proof_2]) = puzzle_data();
    let (instance1, proof1) = instance_and_proof_1;
    let (instance2, proof2) = instance_and_proof_2;
    assert!(verify(&ck, &instance1, &proof1));
    assert!(verify(&ck, &instance2, &proof2));

    let (a, comm_a_rand): (Vec<Fr>, Fr) = {
        // Your solution here!
        todo!()
    };
    assert_eq!(
        ck.commit_with_explicit_randomness(&a, comm_a_rand),
        instance1.comm_a
    );
    assert_eq!(
        ck.commit_with_explicit_randomness(&a, comm_a_rand),
        instance2.comm_a
    );
}