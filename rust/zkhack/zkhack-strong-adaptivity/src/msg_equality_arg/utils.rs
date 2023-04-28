use ark_ed_on_bls12_381::Fr;
use ark_ff::PrimeField;
use ark_serialize::{CanonicalSerialize, CanonicalSerializeHashExt};

pub fn b2s_hash_to_field<C: CanonicalSerialize>(input: &C) -> Fr {
    let bytes = input.hash::<blake2::Blake2s>();
    Fr::from_le_bytes_mod_order(&bytes)
}
