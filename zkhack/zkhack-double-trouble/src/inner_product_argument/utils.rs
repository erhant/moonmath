use ark_ed_on_bls12_381::Fr;
use ark_ff::PrimeField;
use ark_serialize::{CanonicalSerialize, CanonicalSerializeHashExt};
use ark_std::UniformRand;
use rand::Rng;

use crate::{CommitKey, Instance, ProofCommitment};

pub fn b2s_hash_to_field<C: CanonicalSerialize>(input: &C) -> Fr {
    let bytes = input.hash::<blake2::Blake2s>();
    Fr::from_le_bytes_mod_order(&bytes)
}

pub fn challenge(ck: &CommitKey, instance: &Instance, proof_comm: &ProofCommitment) -> Fr {
    b2s_hash_to_field(&(ck.clone(), instance.clone(), proof_comm.clone()))
}

pub fn dot_product(a: &[Fr], b: &[Fr]) -> Fr {
    a.iter().zip(b).map(|(a, b)| *a * b).sum()
}

pub fn sample_vector<U: UniformRand, R: Rng>(size: usize, rng: &mut R) -> Vec<U> {
    let mut vec = Vec::with_capacity(size);
    for _ in 0..size {
        vec.push(U::rand(rng))
    }
    vec
}
