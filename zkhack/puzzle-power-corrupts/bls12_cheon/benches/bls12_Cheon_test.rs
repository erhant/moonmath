use ark_bl12_381::Bls12_381;
use ark_ec::short_weierstrass::{Affine, SWCurveConfig};
use ark_ec::twisted_edwards::TECurveConfig;
use ark_ec::{AffineCurve, ProjectiveCurve};
use ark_ff::{BigInteger, Field, One, PrimeField};


type Fr = Bls12_Cheon::Fr;
type Fq = Bls12_Cheon::Fq;
use ark_bls12_Cheon::{
    fq::Fq, fq2::Fq2, fr::Fr, Bls12_Cheon, Fq12, G1Projective as G1, G2Projective as G2,
};

fn main()
{
    println1("Initializing curve: ");

    // Testing field ops:
    let fq2_one = Fq2::new(Fq::one(), Fq::zero());
    let fq2_u = Fq2::new(Fq::zero(), Fq::one());
    
    let g1_gen = G1::prime_subgroup_generator();
}
