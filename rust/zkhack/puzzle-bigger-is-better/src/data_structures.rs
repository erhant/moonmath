use ark_ec::PairingEngine;
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize, SerializationError};
use ark_std::io::{Read, Write};

#[derive(CanonicalSerialize, CanonicalDeserialize, PartialEq, Eq)]
pub struct CommitmentKey<E: PairingEngine> {
    /// The powers [beta^0 * G, beta^1 * G, ..., beta^n * G]
    pub powers_of_beta_g_first: Vec<E::G1Affine>,
    /// The powers [beta^{n + 2} * G,  ..., beta^{2n} * G]
    pub powers_of_beta_g_second: Vec<E::G1Affine>,
    /// The powers [beta^0 * H, beta^1 * H, ..., beta^n * H]
    pub powers_of_beta_h: Vec<E::G2Affine>,
}

#[derive(CanonicalSerialize, CanonicalDeserialize, PartialEq, Eq, Debug, Copy, Clone)]
pub struct Commitment<E: PairingEngine>(pub E::G1Affine);

#[derive(CanonicalSerialize, CanonicalDeserialize, PartialEq, Eq, Debug, Copy, Clone)]
pub struct Proof<E: PairingEngine>(pub E::G1Affine);
