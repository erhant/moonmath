use ark_ec::{msm::VariableBaseMSM, AffineCurve, PairingEngine};
use ark_ff::PrimeField;
use ark_poly::{univariate::DensePolynomial, Polynomial, UVPolynomial};
use ark_std::Zero;
use core::marker::PhantomData;

use crate::data_structures::*;

pub struct ILV<E: PairingEngine>(PhantomData<E>);

impl<E: PairingEngine> ILV<E> {
    pub fn commit(ck: &CommitmentKey<E>, input: &[E::Fr]) -> Commitment<E> {
        let input_ints = input.iter().map(|x| x.into_repr()).collect::<Vec<_>>();
        Commitment(
            VariableBaseMSM::multi_scalar_mul(&ck.powers_of_beta_g_first[1..], &input_ints).into(),
        )
    }

    pub fn open(ck: &CommitmentKey<E>, a: &[E::Fr], b: &[E::Fr]) -> Proof<E> {
        let dim = a.len();
        let inner_product = a.iter().zip(b.iter()).map(|(&a, b)| a * b).sum::<E::Fr>();

        let mut a_coeffs = Vec::with_capacity(a.len() + 1);
        a_coeffs.push(E::Fr::zero());
        a_coeffs.extend_from_slice(a);
        let a_poly = DensePolynomial::from_coefficients_vec(a_coeffs);
        assert_eq!(a_poly.degree(), dim);
        assert_eq!(a_poly.coeffs[0], E::Fr::zero());

        let mut b_rev = b.to_vec();
        b_rev.push(E::Fr::zero());
        b_rev.reverse();
        let b_poly = DensePolynomial::from_coefficients_vec(b_rev);
        assert_eq!(b_poly.degree(), dim);
        assert_eq!(b_poly.coeffs[0], E::Fr::zero());
        assert_eq!(a_poly.degree(), b_poly.degree());

        let mut product = &a_poly * &b_poly;
        assert_eq!(product.coeffs[0], E::Fr::zero());
        assert_eq!(product.coeffs[dim + 1], inner_product);
        assert_eq!(product.degree(), 2 * dim);

        product.coeffs[dim + 1] -= inner_product;

        let product_coeffs = product
            .coeffs
            .iter()
            .map(|x| x.into_repr())
            .collect::<Vec<_>>();

        // We have to compute the proof piece wise since the (dim + 1)-th coefficient
        // of the product is zero.
        let first_part = VariableBaseMSM::multi_scalar_mul(
            &ck.powers_of_beta_g_first,
            &product_coeffs[..(dim + 1)],
        );
        let second_part = VariableBaseMSM::multi_scalar_mul(
            &ck.powers_of_beta_g_second,
            &product_coeffs[(dim + 2)..],
        );
        let proof = first_part + second_part;
        Proof(proof.into())
    }

    pub fn verify(
        ck: &CommitmentKey<E>,
        cm: &Commitment<E>,
        b: &[E::Fr],
        claimed_inner_product: E::Fr,
        proof: &Proof<E>,
    ) -> bool {
        let dim = b.len();

        let mut b_rev = b.to_vec();
        b_rev.push(E::Fr::zero());
        b_rev.reverse();
        let b_rev = b_rev.iter().map(|x| x.into_repr()).collect::<Vec<_>>();
        assert_eq!(b_rev.len(), ck.powers_of_beta_h.len());
        let b_comm = VariableBaseMSM::multi_scalar_mul(&ck.powers_of_beta_h, &b_rev).into();

        let e1 = E::pairing(proof.0, ck.powers_of_beta_h[0]);
        let e2 = E::pairing(cm.0, b_comm);
        let e3 = E::pairing(
            ck.powers_of_beta_g_first[dim].mul(claimed_inner_product),
            ck.powers_of_beta_h[1],
        );
        e1 * e3 == e2
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use ark_bls12_381::*;
    use ark_serialize::CanonicalDeserialize;
    use ark_std::UniformRand;

    #[test]
    fn test_ilv() {
        let mut rng = ark_std::test_rng();
        let dim = 512;
        let a = (0..dim).map(|_| Fr::rand(&mut rng)).collect::<Vec<_>>();
        let b = (0..dim).map(|_| Fr::rand(&mut rng)).collect::<Vec<_>>();
        let ck = CommitmentKey::<Bls12_381>::deserialize_unchecked(crate::SRS).unwrap();
        let cm = ILV::commit(&ck, &a);
        let proof = ILV::open(&ck, &a, &b);
        let inner_product = a.iter().zip(b.iter()).map(|(&a, b)| a * b).sum::<Fr>();
        assert!(ILV::verify(&ck, &cm, &b, inner_product, &proof));
    }
}
