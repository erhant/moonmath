use ark_ff::{fields::*, MontFp};

use crate::*;

pub type Fq6 = Fp6<Fq6Config>;

#[derive(Clone, Copy)]
pub struct Fq6Config;

impl Fp6Config for Fq6Config {
    type Fp2Config = Fq2Config;

    /// NONRESIDUE = (0 + U)
    const NONRESIDUE: Fq2 = Fq2::new(Fq::ZERO, Fq::ONE);

    const FROBENIUS_COEFF_FP6_C1: &'static [Fq2] = &[
        // Fp2::NONRESIDUE^(((q^0) - 1) / 3)
        Fq2::new(
            Fq::ONE,
            Fq::ZERO,
        ),
        // Fp2::NONRESIDUE^(((q^1) - 1) / 3)
        Fq2::new(
            MontFp!("1071464894672928101371364357925263783348725381978014000658379656413727670822120910586541077416512"),
            Fq::ZERO,
        ),
        // Fp2::NONRESIDUE^(((q^2) - 1) / 3)
        Fq2::new(
            MontFp!("1071464894672928101371364357925263783348725381978014000658379656413727670822120910586541077416511"),
            Fq::ZERO,
        ),
        // Fp2::NONRESIDUE^(((q^3) - 1) / 3)
        Fq2::new(
            MontFp!("5739212180072054691886037078074598258234828766015465682035977006377650233269727206694786492328072119776769214299496"),
            Fq::ZERO,
        ),
        // Fp2::NONRESIDUE^(((q^4) - 1) / 3)
        Fq2::new(
            MontFp!("5739212180072054690814572183401670156863464408090201898687251624399636232611347550281058821505951209190228136882985"),
            Fq::ZERO,
        ),
        // Fp2::NONRESIDUE^(((q^5) - 1) / 3)
        Fq2::new(
            MontFp!("5739212180072054690814572183401670156863464408090201898687251624399636232611347550281058821505951209190228136882986"),
            Fq::ZERO,
        ),
];

    #[rustfmt::skip]
    const FROBENIUS_COEFF_FP6_C2: &'static [Fq2] = &[
        // Fq2(u + 1)**(((2q^0) - 2) / 3)
        Fq2::new(
            Fq::ONE,
            Fq::ZERO,
        ),
        // Fq2(u + 1)**(((2q^1) - 2) / 3)
        Fq2::new(
            MontFp!("1071464894672928101371364357925263783348725381978014000658379656413727670822120910586541077416511"),
            Fq::ZERO,
        ),
        // Fq2(u + 1)**(((2q^2) - 2) / 3)
        Fq2::new(
            MontFp!("5739212180072054690814572183401670156863464408090201898687251624399636232611347550281058821505951209190228136882985"),
            Fq::ZERO,
        ),
        // Fq2(u + 1)**(((2q^3) - 2) / 3)
        Fq2::new(
            MontFp!("1"),
            Fq::ZERO,
        ),
        // Fq2(u + 1)**(((2q^4) - 2) / 3)
        Fq2::new(
            MontFp!("1071464894672928101371364357925263783348725381978014000658379656413727670822120910586541077416511"),
            Fq::ZERO,
        ),
        // Fq2(u + 1)**(((2q^5) - 2) / 3)
        Fq2::new(
            MontFp!("5739212180072054690814572183401670156863464408090201898687251624399636232611347550281058821505951209190228136882985"),
            Fq::ZERO,
        ),
    ];

    /* This is again specialized for bls12_381's nonresidue value of 1+u,
     * hence commenting!
    /// Multiply this element by the quadratic nonresidue 1 + u.
    /// Make this generic.
    fn mul_fp2_by_nonresidue_in_place(fe: &mut Fq2) -> &mut Fq2 {
        let t0 = fe.c0;
        fe.c0 -= &fe.c1;
        fe.c1 += &t0;
        fe
    }
    */
}
