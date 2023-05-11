use ark_bls12_381::{G1Affine, G1Projective};
use ark_crypto_primitives::crh::{
    pedersen::{Window, CRH},
    CRH as CRHScheme,
};
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;

#[derive(Clone)]
struct ZkHackPedersenWindow {}

impl Window for ZkHackPedersenWindow {
    const WINDOW_SIZE: usize = 1;
    const NUM_WINDOWS: usize = 256;
}

pub fn hash_to_curve(msg: &[u8]) -> (Vec<u8>, G1Affine) {
    let rng_pedersen = &mut ChaCha20Rng::from_seed([
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1,
    ]);
    let parameters = CRH::<G1Projective, ZkHackPedersenWindow>::setup(rng_pedersen).unwrap();
    let b2hash = blake2s_simd::blake2s(msg);
    (
        b2hash.as_bytes().to_vec(),
        CRH::<G1Projective, ZkHackPedersenWindow>::evaluate(&parameters, b2hash.as_bytes())
            .unwrap(),
    )
}
