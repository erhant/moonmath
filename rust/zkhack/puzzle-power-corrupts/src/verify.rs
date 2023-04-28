use ark_bls12_cheon::{Fr, G1Projective as G1};
use ark_std::ops::Mul;

pub fn verify(P: G1, tau_P: G1, tau_128: i128) -> bool {
    let tau = Fr::from(tau_128);
    return tau_P == P.mul(Fr::from(tau));
}
