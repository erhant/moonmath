use ark_ec::{AffineCurve, ProjectiveCurve};
use ark_ed_on_bls12_381::{EdwardsAffine as GAffine, Fr};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize, SerializationError};
use ark_std::io::{Read, Write};
use rand::Rng;

pub mod data_structures;
pub use data_structures::*;

pub mod utils;
use utils::*;

#[derive(Clone, CanonicalSerialize, CanonicalDeserialize)]
pub struct Instance {
    pub comm_a: GAffine,
    pub b: Vec<Fr>,
}

pub struct Witness {
    pub a: Vec<Fr>,
    pub comm_a_rand: Fr,
}

pub fn prove<R: Rng>(ck: &CommitKey, instance: &Instance, witness: &Witness, rng: &mut R) -> Proof {
    let a = &witness.a;
    let b = &instance.b;
    debug_assert_eq!(
        ck.commit_with_explicit_randomness(a, witness.comm_a_rand),
        instance.comm_a
    );

    let r = sample_vector(b.len(), rng);
    let r_dot_b = dot_product(&r, b);
    let a_dot_b = dot_product(a, b);
    let (comm_r, comm_r_rand) = ck.commit_with_rng(&r, rng);
    let (comm_1, comm_1_rand) = ck.commit_with_rng(&[a_dot_b], rng);
    let (comm_2, comm_2_rand) = ck.commit_with_rng(&[r_dot_b], rng);
    let commitment = ProofCommitment {
        comm_r,
        comm_1,
        comm_2,
    };

    let challenge = challenge(ck, instance, &commitment);

    let s = a.iter().zip(r).map(|(a, r)| *a + challenge * r).collect();
    let u = witness.comm_a_rand + challenge * comm_r_rand;
    let t = comm_1_rand + challenge * comm_2_rand;
    let response = ProofResponse { s, u, t };

    Proof {
        commitment,
        response,
    }
}

pub fn verify(ck: &CommitKey, instance: &Instance, proof: &Proof) -> bool {
    let b = &instance.b;
    let ProofCommitment {
        comm_r,
        comm_1,
        comm_2,
    } = &proof.commitment;
    let ProofResponse { s, u, t } = &proof.response;

    let challenge = challenge(ck, instance, &proof.commitment);

    let check1 = ck.commit_with_explicit_randomness(s, *u)
        == comm_r.mul(challenge).add_mixed(&instance.comm_a);
    let check2 = ck.commit_with_explicit_randomness(&[dot_product(s, b)], *t)
        == comm_2.mul(challenge).add_mixed(comm_1);
    check1 && check2
}

#[test]
fn test() {
    let ck = CommitKey::sample(32);
    let rng = &mut rand::thread_rng();
    for i in 0..100 {
        let a = sample_vector(32, rng);
        let b = sample_vector(32, rng);
        let (comm_a, comm_a_rand) = ck.commit_with_rng(&a, rng);
        let instance = Instance { comm_a, b };
        let witness = Witness { a, comm_a_rand };
        let proof = prove(&ck, &instance, &witness, rng);
        assert!(verify(&ck, &instance, &proof), "Iteration {} failed", i);
    }
}



// pub fn prove(a: &[Fr], msg: &[u8]) -> Signature {
//     let nonce = deterministic_nonce(&sk.0, &msg);
//     // Commit to the nonce
//     let commitment = EdwardsAffine::prime_subgroup_generator().mul(nonce).into_affine();

//     // Hash the commitment to obtain challenge
//     let mut commitment_bytes = msg.to_vec();
//     commitment.serialize( &mut commitment_bytes).unwrap();
//     let challenge = b2s_hash_to_field(&commitment_bytes);
//     assert_eq!(msg.len() + 32, commitment_bytes.len());

//     // Compute response
//     let response = nonce - sk.to_field_element() * challenge;

//     Signature { challenge, response }
// }

// pub fn verify(pk: &PublicKey, msg: &[u8], signature: &Signature) -> bool {
//     let tmp1 = EdwardsAffine::prime_subgroup_generator().mul(signature.response);
//     let tmp2 = pk.0.mul(signature.challenge);
//     // Recompute claimed commitment:
//     let claimed_commitment = tmp1 + tmp2;
//     let mut claimed_commitment_bytes = msg.to_vec();
//     claimed_commitment.serialize( &mut claimed_commitment_bytes).unwrap();
//     assert_eq!(msg.len() + 32, claimed_commitment_bytes.len());

//     // Check that hash of the claimed commitment matches the challenge in the signature
//     signature.challenge == b2s_hash_to_field(&claimed_commitment_bytes)
// }

// pub fn deterministic_nonce(sk: &[u8], msg: &[u8]) -> Fr {
//     let sk_hash = b2s_hash_to_field(sk);
//     let msg_hash = b2s_hash_to_field(msg);
//     sk_hash + msg_hash
// }

// #[test]
// fn test_signature_correctness() {
//     use ark_std::rand::{RngCore, thread_rng};
//     let mut rng = thread_rng();
//     for _ in 0..1000 {
//         // generate key pair
//         let mut sk_bytes = [0u8; 64];
//         rng.fill_bytes(&mut sk_bytes);
//         let sk = SecretKey(sk_bytes);
//         let pk = PublicKey(EdwardsAffine::prime_subgroup_generator().mul(sk.to_field_element()).into());

//         let msg = b"zkHack is super fun!";
//         // sign message
//         let signature = sign(&sk, msg);
//         // verify signature
//         assert!(verify(&pk, msg, &signature));
//     }
// }
