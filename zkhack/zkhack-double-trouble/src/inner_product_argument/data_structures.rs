use ark_ec::{msm::VariableBaseMSM, AffineCurve};
use ark_ed_on_bls12_381::{EdwardsAffine as GAffine, EdwardsProjective as GProjective, Fr};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize, SerializationError};
use ark_std::{
    io::{Read, Write},
    UniformRand,
};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaChaRng;

use crate::inner_product_argument::utils::*;

#[derive(Clone, CanonicalSerialize, CanonicalDeserialize)]
pub struct CommitKey {
    pub generators: Vec<GAffine>,
    pub hiding_generator: GAffine,
}

impl CommitKey {
    pub fn sample(size: usize) -> Self {
        let mut rng = ChaChaRng::from_seed(*b"zkHack IPA puzzle for 2021-10-26");
        let generators = sample_vector::<GAffine, _>(size, &mut rng)
            .into_iter()
            .map(Into::into)
            .collect();
        let hiding_generator = GProjective::rand(&mut rng).into();
        Self {
            generators,
            hiding_generator,
        }
    }

    pub fn commit_with_explicit_randomness(&self, msg: &[Fr], randomness: Fr) -> GAffine {
        let msg = msg.iter().map(|f| (*f).into()).collect::<Vec<_>>();
        let c = VariableBaseMSM::multi_scalar_mul(&self.generators[..msg.len()], &msg);
        (c + self.hiding_generator.mul(randomness)).into()
    }

    pub fn commit_with_rng<R: Rng>(&self, msg: &[Fr], rng: &mut R) -> (GAffine, Fr) {
        let r = Fr::rand(rng);
        (self.commit_with_explicit_randomness(msg, r), r)
    }
}

#[derive(Clone, CanonicalSerialize, CanonicalDeserialize)]
pub struct ProofCommitment {
    pub comm_r: GAffine,
    pub comm_1: GAffine,
    pub comm_2: GAffine,
}

#[derive(Clone, CanonicalSerialize, CanonicalDeserialize)]
pub struct ProofResponse {
    pub s: Vec<Fr>,
    pub u: Fr,
    pub t: Fr,
}

#[derive(Clone, CanonicalSerialize, CanonicalDeserialize)]
pub struct Proof {
    pub commitment: ProofCommitment,
    pub response: ProofResponse,
}
