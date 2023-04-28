pub mod data_structures;
pub mod error;
pub mod prover;
pub mod rng;
pub mod verifier;

use ark_bls12_381::{Bls12_381, Fr as F};
use ark_ff::Zero;
use ark_poly::{
    univariate::DensePolynomial, EvaluationDomain, GeneralEvaluationDomain, Polynomial,
    UVPolynomial,
};
use ark_poly_commit::{marlin_pc::MarlinKZG10, LabeledPolynomial, PolynomialCommitment};
use ark_std::{rand::rngs::StdRng, test_rng};
use blake2::Blake2s;
use prompt::{puzzle, welcome};
use rand_chacha::ChaChaRng;
use rng::SimpleHashFiatShamirRng;

use crate::{data_structures::Statement, prover::prove, verifier::verify};

pub const PROTOCOL_NAME: &'static [u8] = b"GEOMETRY-SUMCHECK";
pub type PC = MarlinKZG10<Bls12_381, DensePolynomial<F>>;
type FS = SimpleHashFiatShamirRng<Blake2s, ChaChaRng>;

fn main() {
    welcome();
    puzzle(PUZZLE_DESCRIPTION);

    let domain_size = 16;
    let domain = GeneralEvaluationDomain::new(domain_size).unwrap();
    let max_degree = 30;

    let mut rng = test_rng();
    let srs = PC::setup(max_degree, None, &mut rng).unwrap();

    let (ck, vk) = PC::trim(&srs, max_degree, 1, Some(&[domain_size - 2])).unwrap();

    let coeffs = vec![
        F::from(123312u64),
        F::from(124151231u64),
        F::from(1190283019u64),
        F::from(19312315u64),
        F::from(312423151u64),
        F::from(61298741u64),
        F::from(132151231u64),
        F::from(1321512314u64),
        F::from(721315123151u64),
        F::from(783749123u64),
        F::from(2135123151u64),
        F::from(312512314u64),
        F::from(23194890182314u64),
        F::from(321514231512u64),
        F::from(321451231512u64),
        F::from(823897129831u64),
        F::from(908241231u64),
        F::from(9837249823u64),
        F::from(982398741823u64),
        F::from(3891748912u64),
        F::from(21389749812u64),
        F::from(891724876431u64),
        F::from(213145213u64),
        F::from(32897498123u64),
        F::from(3219851289231u64),
        F::from(2184718923u64),
        F::from(31245123131431u64),
        F::from(36712398759812u64),
        F::from(8724876123u64),
        F::from(89783927412u64),
        F::from(8723498123u64),
    ];
    let f = DensePolynomial::from_coefficients_slice(&coeffs);

    // begin sanity
    let mut real_sum = F::zero();
    for h in domain.elements() {
        real_sum += f.evaluate(&h);
    }
    assert_ne!(real_sum, F::zero());
    // end sanity

    let sum = F::zero();

    let f = LabeledPolynomial::new("f".into(), f.clone(), None, Some(1));
    let (f_commitment, f_rand) = PC::commit(&ck, &[f.clone()], Some(&mut rng)).unwrap();

    let statement = Statement {
        domain,
        f: f_commitment[0].commitment().clone(),
        sum,
    };

    let proof = prove::<F, PC, FS, StdRng>(&ck, &statement, &f, &f_rand[0], &mut rng).unwrap();

    let res = verify::<F, PC, FS, StdRng>(&vk, &statement, &proof, &mut rng);
    assert_eq!(true, res.is_ok());
}

const PUZZLE_DESCRIPTION: &str = "\
Bob has designed a new private payments protocol design, where every note comes with a secret 
polynomial f whose sum over a specific set is zero. This is enforced using a sumcheck protocol.
Once a note is spent, f is modified to a different polynomial whose sum isn't zero. One day, 
after an interesting conversation with her friends, Alice got an idea for an attack that can 
potentially allow her to double spend notes.

Alice successfully double spent a note. Can you figure out how she did it?

Be very careful, if the verifier somehow learns the sum of the modified f, 
they can deanonymize you.

In the rest of protocol that is not described here, the masking polynomial used by 
the prover is opened twice. Therefore, the masking polynomial cannot be a 
constant polynomial.

To see examples of sumcheck, you can review the protocol described in 
https://github.com/arkworks-rs/marlin/blob/master/diagram/diagram.pdf.
";
