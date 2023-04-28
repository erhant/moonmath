use ark_ff::{FftField, Field, ToBytes};
use ark_poly::{univariate::DensePolynomial, GeneralEvaluationDomain};
use ark_poly_commit::PolynomialCommitment;

pub struct Statement<F: FftField, PC: PolynomialCommitment<F, DensePolynomial<F>>> {
    pub domain: GeneralEvaluationDomain<F>,
    pub f: PC::Commitment,
    pub sum: F,
}

impl<F: FftField, PC: PolynomialCommitment<F, DensePolynomial<F>>> ToBytes for Statement<F, PC> {
    fn write<W: ark_std::io::Write>(&self, mut writer: W) -> ark_std::io::Result<()> {
        self.f.write(&mut writer)?;
        self.sum.write(&mut writer)
    }
}

pub struct Proof<F: Field, PC: PolynomialCommitment<F, DensePolynomial<F>>> {
    pub f_opening: F,
    pub s: PC::Commitment,
    pub s_opening: F,
    pub g: PC::Commitment,
    pub g_opening: F,
    pub h: PC::Commitment,
    pub h_opening: F,
    pub pc_proof: PC::BatchProof,
}
