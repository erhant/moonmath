use ark_ec::{AffineCurve, ProjectiveCurve};
use ark_ed_on_bls12_381::{EdwardsAffine as GAffine, Fr};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize, SerializationError};
use ark_std::{UniformRand, io::{Read, Write}};
use rand::Rng;

pub mod data_structures;
pub use data_structures::*;

pub mod utils;
use utils::*;

#[derive(Clone, CanonicalSerialize, CanonicalDeserialize)]
pub struct Instance {
    pub comm_1: GAffine,
    pub comm_2: GAffine,
}

pub struct Witness {
    pub a: Fr,
    pub r_1: Fr,
    pub r_2: Fr,
}

pub fn prove<R: Rng>(ck: &CommitKey, instance: &Instance, witness: &Witness, rng: &mut R) -> Proof {
    let a = witness.a;
    debug_assert_eq!(
        ck.commit_with_explicit_randomness(a, witness.r_1),
        instance.comm_1
    );
    debug_assert_eq!(
        ck.commit_with_explicit_randomness(a, witness.r_2),
        instance.comm_2
    );

    let r = Fr::rand(rng);
    let (comm_rho, rho) = ck.commit_with_rng(r, rng);
    let (comm_tau, tau) = ck.commit_with_rng(r, rng);
    let commitment = ProofCommitment {
        comm_rho,
        comm_tau,
    };

    let challenge = b2s_hash_to_field(&(*ck, commitment));

    let s = r + challenge * a;
    let u = rho + challenge * witness.r_1;
    let t = tau + challenge * witness.r_2;
    let response = ProofResponse { s, u, t };

    Proof {
        commitment,
        response,
    }
}

pub fn verify(ck: &CommitKey, instance: &Instance, proof: &Proof) -> bool {
    let c_1 = instance.comm_1;
    let c_2 = instance.comm_2;
    let ProofCommitment {
        comm_rho,
        comm_tau,
    } = proof.commitment;
    let ProofResponse { s, u, t } = proof.response;

    let challenge = b2s_hash_to_field(&(*ck, proof.commitment));

    // Check s * G + u * H == C_rho + challenge * C_1
    let check1 = ck.commit_with_explicit_randomness(s, u)
        == c_1.mul(challenge).add_mixed(&comm_rho);
    // Check s * G + t * H == C_tau + challenge * C_2

    let check2 = ck.commit_with_explicit_randomness(s, t)
        == c_2.mul(challenge).add_mixed(&comm_tau);
    check1 && check2
}

#[test]
fn test() {
    let ck = CommitKey::sample();
    let rng = &mut rand::thread_rng();
    for i in 0..100 {
        let a = Fr::rand(rng);
        let (comm_1, r_1) = ck.commit_with_rng(a, rng);
        let (comm_2, r_2) = ck.commit_with_rng(a, rng);
        let instance = Instance { comm_1, comm_2 };
        let witness = Witness { a, r_1, r_2 };
        let proof = prove(&ck, &instance, &witness, rng);
        assert!(verify(&ck, &instance, &proof), "Iteration {} failed", i);
    }
}
