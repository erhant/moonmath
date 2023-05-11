use ark_ec::PairingEngine;
use ark_serialize::CanonicalSerialize;
use ark_std::{rand::SeedableRng, UniformRand, Zero};
use blake2::Digest;

use crate::{
    algorithms::ILV,
    data_structures::{Commitment, CommitmentKey, Proof},
};

pub struct Attack<E: PairingEngine> {
    /// The vector that will be committed.
    pub a: Vec<E::Fr>,
    /// Commitment to `a`.
    pub commitment: Commitment<E>,
    /// The claimed inner product of `a` and `b := Hash(commitment)`, which differs
    /// from the actual inner product.
    pub claimed_inner_product: E::Fr,
    /// An unsound proof that `a` and `b` have inner product `claimed_inner_product`.
    pub proof: Proof<E>,
}

impl<E: PairingEngine> Attack<E> {
    pub fn assert_attack_works(&self, ck: &CommitmentKey<E>, dim: usize) {
        assert_eq!(self.a.len(), dim);
        assert_eq!(self.commitment, ILV::commit(&ck, &self.a));
        let b = hash(self.commitment, dim);
        let actual_inner_product = self
            .a
            .iter()
            .zip(b.iter())
            .map(|(&a, b)| a * b)
            .sum::<E::Fr>();
        assert!(ILV::verify(
            &ck,
            &self.commitment,
            &b,
            self.claimed_inner_product,
            &self.proof
        ));
        assert_ne!(actual_inner_product, self.claimed_inner_product);
    }
}

pub fn hash<E: PairingEngine>(commitment: Commitment<E>, dim: usize) -> Vec<E::Fr> {
    let mut commitment_bytes = vec![];
    commitment.serialize(&mut commitment_bytes).unwrap();
    let seed: [u8; 32] = blake2::Blake2s::digest(&commitment_bytes)
        .as_slice()
        .try_into()
        .unwrap();
    let mut rng = rand_chacha::ChaChaRng::from_seed(seed);
    let mut b = vec![E::Fr::zero(); dim];
    for i in 0..dim {
        b[i] = E::Fr::rand(&mut rng);
    }
    b
}
