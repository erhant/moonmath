pub mod attack;
pub mod utils;
pub mod verify;

use ark_bls12_cheon::{Fq, Fq2, Fr, G1Projective as G1, G2Projective as G2, Parameters};
use ark_ec::bls12;
use ark_ff::MontFp;

use prompt::{puzzle, welcome};

use crate::{attack::attack, verify::verify};

pub type G2Affine = bls12::G2Affine<crate::Parameters>;
pub type G1Affine = bls12::G1Affine<crate::Parameters>;

fn main() {
    welcome();
    puzzle(PUZZLE_DESCRIPTION);

    // P is the generator for G1
    let P_x: Fq = MontFp!("4366845981406663127346140105392043296067620632748305894915559567990751463871846461571751242076416842353760718219463");
    let P_y: Fq = MontFp!("2322936086289479068066490612801140015408721761579169972917817367391045591350600034547617432914931369293155123935728");
    let P = G1::from(G1Affine::new_unchecked(P_x, P_y));

    // tau_P is tau * P
    let tau_P_x: Fq = MontFp!("4098523714512767373909357151251054769972251070043521443432671846694698445310918888674563384159054413429407635337545");
    let tau_P_y: Fq = MontFp!("5104199250567875649831070757916425527562742910121942289068977475282886267374972984853923971702383949149250750103817");
    let tau_P = G1::from(G1Affine::new_unchecked(tau_P_x, tau_P_y));

    // tau_d1_P is (tau ^ d1) * P
    let tau_d1_P_x: Fq = MontFp!("5666793736468521094298738103921693621508309431638529348089160865885867964805742107934338916926827890667749984768215");
    let tau_d1_P_y: Fq = MontFp!("3326251098281288448602352180414320622119338868949804322483594574847275370379159993188118130786632123868776051955196");
    let tau_d1_P = G1::from(G1Affine::new_unchecked(tau_d1_P_x, tau_d1_P_y));

    // Q is the generator for G2
    let Q_c0_x: Fq = MontFp!("2847190178490156899798643792842723617787968359868175140038826869144776012793105029391523604954249120667126821536281");
    let Q_c1_x: Fq = MontFp!("1513797577242500304678874752065526230408447782356629533374984043360635354098197307045487457331199798062484459984831");
    let Q_x: Fq2 = Fq2::new(Q_c0_x, Q_c1_x);
    let Q_c0_y: Fq = MontFp!("2398127858646538650279262747029238501121661957103909673770298065006753715123740323569605568913154172079135187452386");
    let Q_c1_y: Fq = MontFp!("5444946257649901533268220138726124417824817651440748374257708320447300055543369665159277001725118567443194417165086");
    let Q_y: Fq2 = Fq2::new(Q_c0_y, Q_c1_y);
    let Q = G2::from(G2Affine::new_unchecked(Q_x, Q_y));

    // tau_d2_Q is (tau ^ d2) * Q
    let tau_d2_Q_c0_x: Fq = MontFp!("3536383419772898871062064633012296862124372086039789534814905834326827967479599778599887194095392165550880125330266");
    let tau_d2_Q_c1_x: Fq = MontFp!("2315075704417849395497347082310199859284883937672695000597201154920791698799875018503579607990866594389648640170976");
    let tau_d2_Q_x: Fq2 = Fq2::new(tau_d2_Q_c0_x, tau_d2_Q_c1_x);
    let tau_d2_Q_c0_y: Fq = MontFp!("58522461936731088989461032245338237080030815519467180488197672431529427745827070450637290003234818635632376291077");
    let tau_d2_Q_c1_y: Fq = MontFp!("200313434320582884299950030908390796161004965251373896142196467499133624968316891420231529223691679020778320981956");
    let tau_d2_Q_y: Fq2 = Fq2::new(tau_d2_Q_c0_y, tau_d2_Q_c1_y);
    let tau_d2_Q = G2::from(G2Affine::new_unchecked(tau_d2_Q_x, tau_d2_Q_y));

    let tau = attack(P, tau_P, tau_d1_P, Q, tau_d2_Q);

    assert_eq!(true, verify(P, tau_P, tau));
}

const PUZZLE_DESCRIPTION: &str = r"
Bob has invented a new pairing-friendly elliptic curve, which he wanted to use with Groth16.
For that purpose, Bob has performed a trusted setup, which resulted in an SRS containting
a secret $\tau$ raised to high powers multiplied by a specific generator in both source groups. 
The exact parameters of the curve and part of the output of the setup are described in the 
document linked below.

Alice wants to recover $\tau$ and she noticed a few interesting details about the curve and
the setup. Specifically, she noticed that the sum $d$ of the highest power $d_1$ of $\tau$ in 
$\mathbb{G}_1$ portion of the SRS, meaning the SRS contains an element of the form 
$\tau^{d_1} G_1$ where $G_1$ is a generator of $\mathbb{G}_1$, and the highest power $d_2$ 
of $\tau$ in $\mathbb{G}_2$  divides $q-1$, where $q$ is the order of the groups. 

Additionally, she managed to perform a social engineering attack on Bob and extract the 
following information: if you express $\tau$ as $\tau = 2^{k_0 + k_1((q-1/d))} \mod r$, 
where $r$ is the order of the scalar field, $k_0$ is 51 bits and its fifteen most 
significant bits are 10111101110 (15854 in decimal). That is A < k0 < B where 
A = 1089478584172543 and B = 1089547303649280.

Alice then remembered the Cheon attack...

NOTE: for exponentiating $F_r$ elements, use the `pow_sp` and `pow_sp2` functions in
`utils.rs`.

The parameters of the curve and the setup are available at 
https://gist.github.com/kobigurk/352036cee6cb8e44ddf0e231ee9c3f9b
";
