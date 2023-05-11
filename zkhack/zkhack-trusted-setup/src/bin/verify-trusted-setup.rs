use ark_bls12_381::Fr;
use ark_ec::AffineCurve;
use prompt::{puzzle, welcome};
use std::str::FromStr;
use trusted_setup::data::puzzle_data;
use trusted_setup::PUZZLE_DESCRIPTION;

fn main() {
    welcome();
    puzzle(PUZZLE_DESCRIPTION);
    let (_ts1, _ts2) = puzzle_data();

    /* Your solution here! (s in decimal)*/
    let s = Fr::from_str("0").unwrap();

    assert_eq!(_ts1[0].mul(s), _ts1[1]);
    assert_eq!(_ts2[0].mul(s), _ts2[1]);
}
