use ark_ec::AffineCurve;
use ark_ed_on_bls12_381::{EdwardsAffine as GAffine, EdwardsProjective as GProjective, Fr};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize, SerializationError};
use ark_std::{
    io::{Read, Write},
    UniformRand,
};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaChaRng;

#[derive(Copy, Clone, CanonicalSerialize, CanonicalDeserialize)]
pub struct CommitKey {
    pub message_generator: GAffine,
    pub hiding_generator: GAffine,
}

impl CommitKey {
    pub fn sample() -> Self {
        let mut rng = ChaChaRng::from_seed(*b"zkHack IPA puzzle for 2021-10-26");
        let message_generator = GProjective::rand(&mut rng).into();
        let hiding_generator = GProjective::rand(&mut rng).into();
        Self {
            message_generator,
            hiding_generator,
        }
    }

    pub fn commit_with_explicit_randomness(&self, msg: Fr, randomness: Fr) -> GAffine {
        (self.message_generator.mul(msg) + self.hiding_generator.mul(randomness)).into()
    }

    pub fn commit_with_rng<R: Rng>(&self, msg: Fr, rng: &mut R) -> (GAffine, Fr) {
        let r = Fr::rand(rng);
        (self.commit_with_explicit_randomness(msg, r), r)
    }
}

#[derive(Copy, Clone, CanonicalSerialize, CanonicalDeserialize)]
pub struct ProofCommitment {
    pub comm_rho: GAffine,
    pub comm_tau: GAffine,
}

#[derive(Copy, Clone, CanonicalSerialize, CanonicalDeserialize)]
pub struct ProofResponse {
    pub s: Fr,
    pub u: Fr,
    pub t: Fr,
}

#[derive(Copy, Clone, CanonicalSerialize, CanonicalDeserialize)]
pub struct Proof {
    pub commitment: ProofCommitment,
    pub response: ProofResponse,
}
