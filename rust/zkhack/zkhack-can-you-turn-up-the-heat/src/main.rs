use prompt::{puzzle, welcome};
use winterfell::{
    math::{fields::f128::BaseElement as Felt, FieldElement},
    Air, AirContext, Assertion, TraceTable, FieldExtension, HashFunction, ProofOptions, Prover,
    Serializable, TraceInfo, TransitionConstraintDegree, StarkProof, ByteWriter, EvaluationFrame, Trace
};


// PROOF
// ================================================================================================

/// This is a proof for the correct 32nd term of the Fibonacci sequence; replace it with your 
/// fake proof. NOTE: to build a fake proof you will need to modify Winterfell prover code.
const PROOF: &str = "\
02040000100100000000d3ffffffffffffffffffff010800020104058000160bbea2c10bf0e34d992f72cbada22028e203\
579691245188304c7f48d8c36b240e464669a9b54e12a4dde79f3d02807aa29c87463d655340a0246ee3149c6e4c2f0cbd\
a95819dda258b081d0563f7279f673325ac986a664f39a192c771337ffe25d1fe6f51f5dccb915946ae2a5e4a7b64383be\
f9b99c7f819254234d7ff720000000cecc6da6e6283a9a8e14a577a00662614525f437f391350a965fd3cb4aaf00bde200\
00000107d21d54e54e87cfe89b9b0a4744c5ed9bf4e4bbac1b54f4d41dbe8d3ec736353f356e90e6afa6e72b4da193fa5f\
4f2133f2fe27b536e2bef4fed559bb42bb514f1343a4b20b62abd2c1b259cf0b6f55266d63d79b0c60874027140edd754b\
16bddad0922bbe66fdae289f1042566d4efc452fa8580ca74e95f60fe6153d556975f6dd1c7d4bdd149542b95eb03ee21a\
d08f826c1e4a1f18840da48378d741e68a726f13876cf3b108666f1b681ab90664f69b2ba7dbdadf668671d7ed055ad698\
d6ab0dfc3cc8af6faa2cc4b0546effb4de6323cb95c2600f70c60e74433c90ac20000000daea4b8b46fbc045b7a1e646a0\
b69d19381541d32538bc5360adb7ff8e155a9fe20000000107ec7028f6b714f24ca997219e3c34363cdfc55494d302a7cc\
64a4e40a97bdc8cdba0305bb02c85b9c9534b92e2b348ac620ee823f6bf95af345102929c89af746bf4e5b52cc5efe7f61\
59f416bebbf781cf383ba29ede01cfb05702ec18dac7fa322212fa7fc76102900d5a139ca861ffa46f0d8c495a3bca2515\
702f1e62897020ce8da4ed837cbc43c87cb893611b0e1c7a6bcd46047dcbf55eacf5a81657bf5702cfcac1dc9c92e3d2cf\
02057c8d530d1b29bf49592bfa6cfcbf78c59eb6e07602c629666c7a869e43f26bdb78af0c9cbac256d937029f9ae9b07e\
7035b15a2000774624059187227e84bab59d90ecdc27775facd9ad2ad554e1fbc421b4521e57152ab6c78ba0b923e1d606\
1172de1dd7c812feb7730665864fe254b21bb9d8fb2000b9da5c6d211f33787ee4779dc8cc4e336700558d280cfa6149ea\
9c230d065e2f0140000000b1585f1335e26df64bcac41bbe88b24efbb8f7680032e2987f5e25050dd57b8ceacd6d1a2126\
34d8a9b27ed4307c820b657fb8313e8a88d7406519950d59f45ba200000001057f2438fb9bbadb574be4c2093865962b02\
90ef1ec628458bbb3b6149cdce66ff9fda9b33b10c2ab4516e1efc565fc5af07236fb7c28fa3de02f42992c79b13c509a5\
56ddf15d2665dc8c38e2169d8d823419c07bd8a97b79d2f83d0941ccad27d6a34b1922273423da72855b2c210b9a79705e\
db95ce2a1c5f4fea67649727d40c827c9b169fca98880fbc87e7b310d40f2041eb2be29352cd1f25745bf7c0f20002cfcd\
c55d9373f4a65e4fe085bb5e9324351fbc684414bb26553bccc92a7db15f7cebe745bed43eb24249ca233c72df764ce0de\
b652aa5cc9135b39ac462e8ad5925e9dc868ef4df3b3fe81151d9a4fd71e90f0a6fb3fd46d20d35aa25a60bfd88e935db4\
2665f5a5d97b2594e606e739fb76fe7f858074495608b47b02808eada768263bd299b71205a6f0ff35134aef36576c5fb1\
4cd6ebb99c928186971aa04bd5c95f252270e40060fc61610c79f0b24c48c8ba563fc0f911f3fc70d22eafb352ec8f81a7\
ba7d1db640c060cc1e43b52d43edcd98cab2ca40892361b6043e7bf4044edc2dc30bc705ef0828a6337e38dfc72690bae7\
7eb829ad97b19eb16843bf2a7f9f3180ae2a7e75500164bbc16aeccff16f446ee7ff0e4fd3f75898f725e2093694575f25\
0a91a331f671f2d0e177f10336a77800c5bca586d02d7fad498273963c4c0e17e1802c20853e98f82b7c7ac1b71d5aa84f\
cab390b7eca7c3cb10036985334101774a60ab2f39326c1d1ea86349787443f477d13be37703bcc25a21bfd32b358b6b2c\
b849109327d6a6ed3f1d73c561f9eab850015afb5bb48fad7cfb601c5337ac6594686f7875739cab1f5495c24c6927762c\
f07a3a8879ed1c3be8b1874a6b476a0f2a00c4b9987d168a815774682abfc166c98f6554d1a6b7d26b50d16740e2d389d7\
c48273872bdcc9cc79b790e7f6f5cbd06bd27cd9000100000000000000";

// MAIN FUNCTION
// ================================================================================================

pub fn main() {

    welcome();
    puzzle(PUZZLE_DESCRIPTION);

    let n = 32;
    let start = (Felt::ZERO, Felt::ONE);
    let end = fib(start, n);
    assert_eq!(end, Felt::new(832040));

    // deserialize proof
    let proof_bytes = hex::decode(PROOF).unwrap();
    let proof = StarkProof::from_bytes(&proof_bytes).unwrap();
    
    // initialize public inputs; if we set `end` to 832040 (which is the actual 32nd term of the
    // Fibonacci sequence) the current valid proof will pass.
    let pub_inputs = FibInputs { start, end: Felt::new(123) };

    // verify the proof; make sure your fake proof doesn't fail this assertion
    assert!(winterfell::verify::<FibAir>(proof, pub_inputs).is_ok());    
}

// FIBONACCI FUNCTION
// ================================================================================================

fn fib(start: (Felt, Felt), n: usize) -> Felt {
    let mut t0 = start.0;
    let mut t1 = start.1;

    for _ in 0..(n - 1) {
        t1 = t0 + t1;
        core::mem::swap(&mut t0, &mut t1);
    }
    t1
}

// PUBLIC INPUTS
// ================================================================================================

#[derive(Clone)]
struct FibInputs {
    start: (Felt, Felt),
    end: Felt,
}

impl Serializable for FibInputs {
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        target.write(self.start.0);
        target.write(self.start.1);
        target.write(self.end);
    }
}

// FIBONACCI AIR
// ================================================================================================

struct FibAir {
    context: AirContext<Felt>,
    start: (Felt, Felt),
    end: Felt,
}

impl Air for FibAir {
    type BaseField = Felt;
    type PublicInputs = FibInputs;

    fn new(trace_info: TraceInfo, pub_inputs: FibInputs, options: ProofOptions) -> Self {
        let degrees = vec![
            TransitionConstraintDegree::new(1),
            TransitionConstraintDegree::new(1),
        ];
        Self {
            context: AirContext::new(trace_info, degrees, options),
            start: pub_inputs.start,
            end: pub_inputs.end,
        }
    }

    fn evaluate_transition<E: FieldElement<BaseField = Self::BaseField>>(
        &self,
        frame: &EvaluationFrame<E>,
        _periodic_values: &[E],
        result: &mut [E],
    ) {
        let current_state = frame.current();
        let next_state = frame.next();

        // constraints of Fibonacci sequence (2 terms per step):
        // s_{0, i+1} = s_{0, i} + s_{1, i}
        // s_{1, i+1} = s_{1, i} + s_{0, i+1}
        result[0] = next_state[0] - (current_state[0] + current_state[1]);
        result[1] = next_state[1] - (current_state[1] + next_state[0]);
    }

    fn get_assertions(&self) -> Vec<Assertion<Self::BaseField>> {
        let last_step = self.trace_length() - 1;
        vec![
            Assertion::single(0, 0, self.start.0),
            Assertion::single(1, 0, self.start.1),
            Assertion::single(0, last_step, self.end),
        ]
    }

    fn context(&self) -> &AirContext<Self::BaseField> {
        &self.context
    }
}

// FIBONACCI PROVER
// ================================================================================================

struct FibProver {
    options: ProofOptions,
}

impl FibProver {

    pub fn new(options: ProofOptions) -> Self {
        Self { options }
    }

    pub fn build_trace(start: (Felt, Felt), n: usize) -> TraceTable<Felt> {
        
        let mut trace = TraceTable::new(2, n / 2);
        trace.fill(
            |state| {
                state[0] = start.0;
                state[1] = start.1;
            },
            |_, state| {
                state[0] += state[1];
                state[1] += state[0];
            },
        );

        trace
    }

}

impl Prover for FibProver {
    type BaseField = Felt;
    type Air = FibAir;
    type Trace = TraceTable<Felt>;

    fn get_pub_inputs(&self, trace: &Self::Trace) -> FibInputs {
        let last_step = trace.length() - 1;
        FibInputs {
            start: (trace.get(0, 0), trace.get(1, 0)),
            end: trace.get(0, last_step),
        }
    }

    fn options(&self) -> &ProofOptions {
        &self.options
    }
}

// PUZZLE DESCRIPTION
// ================================================================================================

const PUZZLE_DESCRIPTION: &str = "\
Alice built a STARK prover and a verifier for verifying correct computation of the Fibonacci
sequence. Her friends would generate proofs for various terms send them to her. She'd verify
these proofs and every time the proof was accepted by the verifier, she'd feel joy.

One day, she received two proofs attesting to the correct computation of the 32nd term of the
Fibonacci sequence. These proofs claimed different results - but both proofs were accepted by the
verifier. This should not have been possible! Worried, Alice computed the 32nd term of the sequence
manually. It was 832040, but one of the proofs was claiming that the 32nd term was 123! Something
must have gone terribly wrong...

Can you figure out how to crate a fake proof at the same security level as the valid proof received
by Alice?
";
