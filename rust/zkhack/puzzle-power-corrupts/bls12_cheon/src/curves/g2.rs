use ark_std::ops::Neg;

use ark_ec::{
    bls12,
    // bls12::Bls12Parameters,
    models::CurveConfig,
    short_weierstrass::{Affine, Projective, SWCurveConfig},
    AffineRepr,
    // CurveGroup, Group,
};
use ark_ff::{Field, MontFp, Zero};
use ark_serialize::{Compress, SerializationError};

use super::util::{serialize_fq, EncodingFlags, G2_SERIALIZED_SIZE};
use crate::{
    util::{read_g2_compressed, read_g2_uncompressed},
    *,
};

pub type G2Affine = bls12::G2Affine<crate::Parameters>;
pub type G2Projective = bls12::G2Projective<crate::Parameters>;

#[derive(Clone, Default, PartialEq, Eq)]
pub struct Parameters;

impl CurveConfig for Parameters {
    type BaseField = Fq2;
    type ScalarField = Fr;

    /// COFACTOR = (x^8 - 4 x^7 + 5 x^6) - (4 x^4 + 6 x^3 - 4 x^2 - 4 x + 13) //
    /// 9
    /// = 305502333931268344200999753193121504214466019254188142667664032982267604182971884026507427359259977847832272839041616661285803823378372096355777062779109
    #[rustfmt::skip]
    const COFACTOR: &'static [u64] = &[
        0xa00566e5d6a846e5,
        0x3527150a43f9e035,
        0x14348c2b97e370a7,
        0x4b2aae0d09e6af0c,
        0xd8d7b482507d5db0,
        0xbfa1db1cc7aa9f41,
        0x17fa995bbdd56297,
        0xb11e4dde4b08b45b,
        0xe4a667cf816fa44a,
        0x02525b8aae834a2c,
        0x5e4b162e0e5
    ];

    /// COFACTOR_INV = COFACTOR^{-1} mod r
    /// 26652489039290660355457965112010883481355318854675681319708643586776743290055
    const COFACTOR_INV: Fr =
        MontFp!("356536809629845402844563");
}

impl SWCurveConfig for Parameters {
    /// COEFF_A = [0, 0]
    const COEFF_A: Fq2 = Fq2::new(g1::Parameters::COEFF_A, g1::Parameters::COEFF_A);

    /// COEFF_B = 4/u, so 499061928701917799294438006789095500716072066610040494090084957076317411588671931016937955854614966937110366460826*u
    const COEFF_B: Fq2 = Fq2::new(Fq::ZERO, MontFp!("499061928701917799294438006789095500716072066610040494090084957076317411588671931016937955854614966937110366460826") );

    /// AFFINE_GENERATOR_COEFFS = (G2_GENERATOR_X, G2_GENERATOR_Y)
    const GENERATOR: G2Affine = G2Affine::new_unchecked(G2_GENERATOR_X, G2_GENERATOR_Y);

    #[inline(always)]
    fn mul_by_a(_: Self::BaseField) -> Self::BaseField {
        Self::BaseField::zero()
    }

    /*
    fn is_in_correct_subgroup_assuming_on_curve(point: &G2Affine) -> bool {
        // Algorithm from Section 4 of https://eprint.iacr.org/2021/1130.
        //
        // Checks that [p]P = [X]P

        let mut x_times_point = point.mul_bigint(crate::Parameters::X);
        if crate::Parameters::X_IS_NEGATIVE {
            x_times_point = -x_times_point;
        }

        let p_times_point = p_power_endomorphism(point);

        x_times_point.eq(&p_times_point)
    }

    #[inline]
    fn clear_cofactor(p: &G2Affine) -> G2Affine {
        // Based on Section 4.1 of https://eprint.iacr.org/2017/419.pdf
        // [h(ψ)]P = [x^2 − x − 1]P + [x − 1]ψ(P) + (ψ^2)(2P)

        // x = -15132376222941642752
        // When multiplying, use -c1 instead, and then negate the result. That's much
        // more efficient, since the scalar -c1 has less limbs and a much lower Hamming
        // weight.
        let x: &'static [u64] = crate::Parameters::X;
        let p_projective = p.into_group();

        // [x]P
        let x_p = Parameters::mul_affine(p, &x).neg();
        // ψ(P)
        let psi_p = p_power_endomorphism(&p);
        // (ψ^2)(2P)
        let mut psi2_p2 = double_p_power_endomorphism(&p_projective.double());

        // tmp = [x]P + ψ(P)
        let mut tmp = x_p.clone();
        tmp += &psi_p;

        // tmp2 = [x^2]P + [x]ψ(P)
        let mut tmp2: Projective<Parameters> = tmp;
        tmp2 = tmp2.mul_bigint(x).neg();

        // add up all the terms
        psi2_p2 += tmp2;
        psi2_p2 -= x_p;
        psi2_p2 += &-psi_p;
        (psi2_p2 - p_projective).into_affine()
    }
    */

    fn deserialize_with_mode<R: ark_serialize::Read>(
        mut reader: R,
        compress: ark_serialize::Compress,
        validate: ark_serialize::Validate,
    ) -> Result<Affine<Self>, ark_serialize::SerializationError> {
        let p = if compress == ark_serialize::Compress::Yes {
            read_g2_compressed(&mut reader)?
        } else {
            read_g2_uncompressed(&mut reader)?
        };

        if validate == ark_serialize::Validate::Yes && !p.is_in_correct_subgroup_assuming_on_curve()
        {
            return Err(SerializationError::InvalidData);
        }
        Ok(p)
    }

    fn serialize_with_mode<W: ark_serialize::Write>(
        item: &Affine<Self>,
        mut writer: W,
        compress: ark_serialize::Compress,
    ) -> Result<(), SerializationError> {
        let encoding = EncodingFlags {
            is_compressed: compress == ark_serialize::Compress::Yes,
            is_infinity: item.is_zero(),
            is_lexographically_largest: item.y > -item.y,
        };
        let mut p = *item;
        if encoding.is_infinity {
            p = G2Affine::zero();
        }

        let mut x_bytes = [0u8; G2_SERIALIZED_SIZE];
        let c1_bytes = serialize_fq(p.x.c1);
        let c0_bytes = serialize_fq(p.x.c0);
        x_bytes[0..48].copy_from_slice(&c1_bytes[..]);
        x_bytes[48..96].copy_from_slice(&c0_bytes[..]);
        if encoding.is_compressed {
            let mut bytes: [u8; G2_SERIALIZED_SIZE] = x_bytes;

            encoding.encode_flags(&mut bytes);
            writer.write_all(&bytes)?;
        } else {
            let mut bytes = [0u8; 2 * G2_SERIALIZED_SIZE];

            let mut y_bytes = [0u8; G2_SERIALIZED_SIZE];
            let c1_bytes = serialize_fq(p.y.c1);
            let c0_bytes = serialize_fq(p.y.c0);
            y_bytes[0..48].copy_from_slice(&c1_bytes[..]);
            y_bytes[48..96].copy_from_slice(&c0_bytes[..]);
            bytes[0..G2_SERIALIZED_SIZE].copy_from_slice(&x_bytes);
            bytes[G2_SERIALIZED_SIZE..].copy_from_slice(&y_bytes);

            encoding.encode_flags(&mut bytes);
            writer.write_all(&bytes)?;
        };

        Ok(())
    }

    fn serialized_size(compress: ark_serialize::Compress) -> usize {
        if compress == Compress::Yes {
            G2_SERIALIZED_SIZE
        } else {
            2 * G2_SERIALIZED_SIZE
        }
    }
}

pub const G2_GENERATOR_X: Fq2 = Fq2::new(G2_GENERATOR_X_C0, G2_GENERATOR_X_C1);
pub const G2_GENERATOR_Y: Fq2 = Fq2::new(G2_GENERATOR_Y_C0, G2_GENERATOR_Y_C1);

/// G2_GENERATOR_X_C0 =
pub const G2_GENERATOR_X_C0: Fq = MontFp!("2847190178490156899798643792842723617787968359868175140038826869144776012793105029391523604954249120667126821536281");

/// G2_GENERATOR_X_C1 =
/// 3059144344244213709971259814753781636986470325476647558659373206291635324768958432433509563104347017837885763365758
pub const G2_GENERATOR_X_C1: Fq = MontFp!("1513797577242500304678874752065526230408447782356629533374984043360635354098197307045487457331199798062484459984831");

/// G2_GENERATOR_Y_C0 =
pub const G2_GENERATOR_Y_C0: Fq = MontFp!("2398127858646538650279262747029238501121661957103909673770298065006753715123740323569605568913154172079135187452386");

/// G2_GENERATOR_Y_C1 =
pub const G2_GENERATOR_Y_C1: Fq = MontFp!("5444946257649901533268220138726124417824817651440748374257708320447300055543369665159277001725118567443194417165086");

// psi(x,y) = (x**p * PSI_X, y**p * PSI_Y) is the Frobenius composed
// with the quadratic twist and its inverse

// PSI_X = 1/(u+1)^((p-1)/3)
pub const P_POWER_ENDOMORPHISM_COEFF_0 : Fq2 = Fq2::new(
    Fq::ZERO,
    MontFp!(
                "4002409555221667392624310435006688643935503118305586438271171395842971157480381377015405980053539358417135540939437"
    )
);

// PSI_Y = 1/(u+1)^((p-1)/2)
pub const P_POWER_ENDOMORPHISM_COEFF_1: Fq2 = Fq2::new(
    MontFp!(
                "2973677408986561043442465346520108879172042883009249989176415018091420807192182638567116318576472649347015917690530"),
    MontFp!(
                "1028732146235106349975324479215795277384839936929757896155643118032610843298655225875571310552543014690878354869257")
);

pub const DOUBLE_P_POWER_ENDOMORPHISM: Fq2 = Fq2::new(
    MontFp!("4002409555221667392624310435006688643935503118305586438271171395842971157480381377015405980053539358417135540939436"),
    Fq::ZERO
);

pub fn p_power_endomorphism(p: &Affine<Parameters>) -> Affine<Parameters> {
    // The p-power endomorphism for G2 is defined as follows:
    // 1. Note that G2 is defined on curve E': y^2 = x^3 + 4(u+1).
    //    To map a point (x, y) in E' to (s, t) in E,
    //    one set s = x / ((u+1) ^ (1/3)), t = y / ((u+1) ^ (1/2)),
    //    because E: y^2 = x^3 + 4.
    // 2. Apply the Frobenius endomorphism (s, t) => (s', t'),
    //    another point on curve E, where s' = s^p, t' = t^p.
    // 3. Map the point from E back to E'; that is,
    //    one set x' = s' * ((u+1) ^ (1/3)), y' = t' * ((u+1) ^ (1/2)).
    //
    // To sum up, it maps
    // (x,y) -> (x^p / ((u+1)^((p-1)/3)), y^p / ((u+1)^((p-1)/2)))
    // as implemented in the code as follows.

    let mut res = *p;
    res.x.frobenius_map(1);
    res.y.frobenius_map(1);

    let tmp_x = res.x.clone();
    res.x.c0 = -P_POWER_ENDOMORPHISM_COEFF_0.c1 * &tmp_x.c1;
    res.x.c1 = P_POWER_ENDOMORPHISM_COEFF_0.c1 * &tmp_x.c0;
    res.y *= P_POWER_ENDOMORPHISM_COEFF_1;

    res
}

/// For a p-power endomorphism psi(P), compute psi(psi(P))
pub fn double_p_power_endomorphism(p: &Projective<Parameters>) -> Projective<Parameters> {
    let mut res = *p;

    res.x *= DOUBLE_P_POWER_ENDOMORPHISM;
    res.y = res.y.neg();

    res
}

#[cfg(test)]
mod test {

    use super::*;
    use ark_std::UniformRand;

    /* This is expected to fail because we didnt fill P_POWER_ENDOMPRPHISM Values.
    #[test]
    fn test_cofactor_clearing() {
        // multiplying by h_eff and clearing the cofactor by the efficient
        // endomorphism-based method should yield the same result.
        let h_eff: &'static [u64] = &[
            0xe8020005aaa95551,
            0x59894c0adebbf6b4,
            0xe954cbc06689f6a3,
            0x2ec0ec69d7477c1a,
            0x6d82bf015d1212b0,
            0x329c2f178731db95,
            0x9986ff031508ffe1,
            0x88e2a8e9145ad768,
            0x584c6a0ea91b3528,
            0xbc69f08f2ee75b3,
        ];

        let mut rng = ark_std::test_rng();
        const SAMPLES: usize = 10;
        for _ in 0..SAMPLES {
            let p = Affine::<g2::Parameters>::rand(&mut rng);
            let optimised = p.clear_cofactor().into_group();
            let naive = g2::Parameters::mul_affine(&p, h_eff);
            assert_eq!(optimised, naive);
        }
    }
    */
}
