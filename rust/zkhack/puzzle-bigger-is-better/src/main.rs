use ark_bls12_381::Bls12_381;
use ark_serialize::CanonicalDeserialize;
use prompt::{puzzle, welcome};

pub mod algorithms;
pub mod data_structures;
use data_structures::*;

pub mod attack;

fn main() {
    welcome();
    puzzle(PUZZLE_DESCRIPTION);
    // Supports committing to vectors of length up to 512.
    let ck = data_structures::CommitmentKey::<Bls12_381>::deserialize_unchecked(SRS).unwrap();
    let attack = attack(&ck, SUPPORTED_DIM);
    attack.assert_attack_works(&ck, SUPPORTED_DIM);
}

pub fn attack<E: ark_ec::PairingEngine>(ck: &CommitmentKey<E>, dim: usize) -> attack::Attack<E> {
    // your code here
    use ark_std::Zero;
    attack::Attack {
        a: vec![],
        commitment: Commitment(E::G1Affine::zero()),
        claimed_inner_product: E::Fr::zero(),
        proof: Proof(E::G1Affine::zero()),
    }
}

const SRS: &'static [u8] = include_bytes!("../ck.srs");
const SUPPORTED_DIM: usize = 512;

const PUZZLE_DESCRIPTION: &str = r"
Bob was catching up on the latest in zkSNARK research, and came across the
Vampire paper [1]. In that paper, he found a reference to an inner-product
commitment scheme [2], which allows committing to a vector and later proving
that its inner-product with another (public) vector is equal to a claimed value.
Bob was intrigued by this scheme, and decided to implement it in Rust.

Bob was delighted with the performance of the resulting implementation, and so
decided to deploy it. The scheme requires a universal Powers-of-Tau-type trusted setup, 
and so Bob generated a SRS using an MPC ceremony.

Things were going smoothly for a while, but then Bob received an anonymous email that 
contained a full break of the scheme! Unfortunately for Bob, the email didn't contain
any details about the break. Can you help Bob figure out the issue, and fix his scheme?

[1]: https://ia.cr/2022/406
[2]: http://www.lsv.fr/Publis/PAPERS/PDF/ILV-imacc11-long.pdf
";
