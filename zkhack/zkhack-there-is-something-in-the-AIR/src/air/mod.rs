use super::{Digest, Felt, FieldElement, Rescue, HASH_CYCLE_LEN, TRACE_WIDTH};
use winterfell::{
    Air, AirContext, Assertion, ByteWriter, EvaluationFrame, ProofOptions, Serializable, TraceInfo,
    TransitionConstraintDegree,
};

mod utils;
use utils::{are_equal, is_binary, is_zero, not, EvaluationResult};

mod rescue;

// SEMAPHORE AIR
// ================================================================================================

/// Defines public inputs for Semaphore AIR.
#[derive(Debug)]
pub struct PublicInputs {
    pub tree_root: Digest,
    pub nullifier: Digest,
    pub topic: Digest,
}

impl Serializable for PublicInputs {
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        target.write(self.tree_root);
        target.write(self.nullifier);
        target.write(self.topic);
    }
}

/// Defines Semaphore AIR.
///
/// To evaluate AIR constraints, we need to know the root of the access set Merkle tree, the
/// nullifier of the signal, and the topic on which the signal was made.
pub struct SemaphoreAir {
    context: AirContext<Felt>,
    tree_root: [Felt; 4],
    nullifier: [Felt; 4],
    topic: [Felt; 4],
}

impl Air for SemaphoreAir {
    type BaseField = Felt;
    type PublicInputs = PublicInputs;

    // CONSTRUCTOR
    // --------------------------------------------------------------------------------------------
    fn new(trace_info: TraceInfo, pub_inputs: PublicInputs, options: ProofOptions) -> Self {
        let degrees = vec![
            // Merkle path hashing
            TransitionConstraintDegree::with_cycles(7, vec![HASH_CYCLE_LEN]),
            TransitionConstraintDegree::with_cycles(7, vec![HASH_CYCLE_LEN]),
            TransitionConstraintDegree::with_cycles(7, vec![HASH_CYCLE_LEN]),
            TransitionConstraintDegree::with_cycles(7, vec![HASH_CYCLE_LEN]),
            TransitionConstraintDegree::with_cycles(7, vec![HASH_CYCLE_LEN]),
            TransitionConstraintDegree::with_cycles(7, vec![HASH_CYCLE_LEN]),
            TransitionConstraintDegree::with_cycles(7, vec![HASH_CYCLE_LEN]),
            TransitionConstraintDegree::with_cycles(7, vec![HASH_CYCLE_LEN]),
            TransitionConstraintDegree::with_cycles(7, vec![HASH_CYCLE_LEN]),
            TransitionConstraintDegree::with_cycles(7, vec![HASH_CYCLE_LEN]),
            TransitionConstraintDegree::with_cycles(7, vec![HASH_CYCLE_LEN]),
            TransitionConstraintDegree::with_cycles(7, vec![HASH_CYCLE_LEN]),
            // nullifier hashing
            TransitionConstraintDegree::with_cycles(7, vec![HASH_CYCLE_LEN]),
            TransitionConstraintDegree::with_cycles(7, vec![HASH_CYCLE_LEN]),
            TransitionConstraintDegree::with_cycles(7, vec![HASH_CYCLE_LEN]),
            TransitionConstraintDegree::with_cycles(7, vec![HASH_CYCLE_LEN]),
            TransitionConstraintDegree::with_cycles(7, vec![HASH_CYCLE_LEN]),
            TransitionConstraintDegree::with_cycles(7, vec![HASH_CYCLE_LEN]),
            TransitionConstraintDegree::with_cycles(7, vec![HASH_CYCLE_LEN]),
            TransitionConstraintDegree::with_cycles(7, vec![HASH_CYCLE_LEN]),
            TransitionConstraintDegree::with_cycles(7, vec![HASH_CYCLE_LEN]),
            TransitionConstraintDegree::with_cycles(7, vec![HASH_CYCLE_LEN]),
            TransitionConstraintDegree::with_cycles(7, vec![HASH_CYCLE_LEN]),
            TransitionConstraintDegree::with_cycles(7, vec![HASH_CYCLE_LEN]),
            // index bits are binary
            TransitionConstraintDegree::new(2),
            // private key equality check
            TransitionConstraintDegree::with_cycles(1, vec![HASH_CYCLE_LEN]),
            TransitionConstraintDegree::with_cycles(1, vec![HASH_CYCLE_LEN]),
            TransitionConstraintDegree::with_cycles(1, vec![HASH_CYCLE_LEN]),
            TransitionConstraintDegree::with_cycles(1, vec![HASH_CYCLE_LEN]),
        ];

        Self {
            context: AirContext::new(trace_info, degrees, options),
            tree_root: pub_inputs.tree_root.into(),
            nullifier: pub_inputs.nullifier.into(),
            topic: pub_inputs.topic.into(),
        }
    }

    // PERIODIC COLUMNS
    // --------------------------------------------------------------------------------------------

    /// Returns a set of periodic columns for Semaphore AIR.
    ///
    /// The columns consist of:
    /// - Hash cycle column, which has a repeating pattern of 7 ones followed by a single zero.
    /// - Key mask column, which has a repeating pattern of a single one followed by 7 zeros.
    /// - 24 round constant columns for Rescue Prime hash function.
    fn get_periodic_column_values(&self) -> Vec<Vec<Felt>> {
        let mut result = vec![HASH_CYCLE_MASK.to_vec(), KEY_CMP_MASK.to_vec()];
        result.append(&mut rescue::get_round_constants());
        result
    }

    // ASSERTIONS
    // --------------------------------------------------------------------------------------------

    /// Returns a set of assertions for Semaphore AIR. These assertions are transformed by the
    /// prover and verifier into boundary constraints.
    ///
    /// These assertions enforce that:
    /// - The trace terminates with Merkle tree root in columns [4, 5, 6, 7].
    /// - Columns [16, 16, 18, 19] at step 7 contain value of the nullifier.
    /// - Topic was inserted into columns [20, 21, 22, 23] at the first step.
    /// - Columns [0, 1, 2, 3] are reset to zeros every 8 steps to (8, 0, 0, 0).
    /// - Columns [12, 13, 14, 15] are set to (8, 0, 0, 0) at the first step.
    fn get_assertions(&self) -> Vec<Assertion<Felt>> {
        let last_step = self.trace_length() - 1;
        vec![
            Assertion::single(4, last_step, self.tree_root[0]),
            Assertion::single(5, last_step, self.tree_root[1]),
            Assertion::single(6, last_step, self.tree_root[2]),
            Assertion::single(7, last_step, self.tree_root[3]),
            Assertion::single(16, 7, self.nullifier[0]),
            Assertion::single(17, 7, self.nullifier[1]),
            Assertion::single(18, 7, self.nullifier[2]),
            Assertion::single(19, 7, self.nullifier[3]),
            Assertion::single(20, 0, self.topic[0]),
            Assertion::single(21, 0, self.topic[1]),
            Assertion::single(22, 0, self.topic[2]),
            Assertion::single(23, 0, self.topic[3]),
        ]
    }

    // TRANSITION CONSTRAINTS
    // --------------------------------------------------------------------------------------------

    /// Evaluates Semaphore transition constraints for two consecutive rows of the execution trace.
    ///
    /// The constraints must have the following properties:
    /// - For all valid transitions between consecutive computation steps, transition constraints
    ///   should evaluation to all zeros.
    /// - For any invalid transition, at least one constraint must evaluate to a non-zero value.
    fn evaluate_transition<E: FieldElement + From<Felt>>(
        &self,
        frame: &EvaluationFrame<E>,
        periodic_values: &[E],
        result: &mut [E],
    ) {
        let current = frame.current();
        let next = frame.next();
        // expected state width is 25 field elements
        debug_assert_eq!(TRACE_WIDTH, current.len());
        debug_assert_eq!(TRACE_WIDTH, next.len());

        // split periodic values into masks and Rescue round constants
        let hash_flag = periodic_values[0];
        let key_cmp_flag = periodic_values[1];
        let ark = &periodic_values[2..];

        // when hash_flag = 1, constraints for Rescue round are enforced for Merkle path
        // hashing and for nullifier hashing
        rescue::enforce_round(
            &mut result[..12],
            &current[..12],
            &next[..12],
            ark,
            hash_flag,
        );
        rescue::enforce_round(
            &mut result[12..24],
            &current[12..24],
            &next[12..24],
            ark,
            hash_flag,
        );

        // for Merkle path hashing, when hash_flag = 0, make sure accumulated hash is placed in the
        // right place in the hash state for the next round of hashing. Specifically: when index
        // bit = 0 accumulated hash must go into columns [4, 5, 6, 7], and when index bit = 0, it must
        // go into columns [8, 9, 10, 11]; also, make sure the capacity columns are reset to ZERO.
        let hash_init_flag = not(hash_flag);
        let bit = next[24];
        let not_bit = not(bit);

        result.agg_constraint(1, hash_init_flag, are_equal(E::from(8u8), next[0]));
        result.agg_constraint(2, hash_init_flag, is_zero(next[1]));
        result.agg_constraint(3, hash_init_flag, is_zero(next[2]));
        result.agg_constraint(4, hash_init_flag, is_zero(next[3]));

        result.agg_constraint(4, hash_init_flag, not_bit * are_equal(current[4], next[4]));
        result.agg_constraint(5, hash_init_flag, not_bit * are_equal(current[5], next[5]));
        result.agg_constraint(6, hash_init_flag, not_bit * are_equal(current[6], next[6]));
        result.agg_constraint(7, hash_init_flag, not_bit * are_equal(current[7], next[7]));

        result.agg_constraint(8, hash_init_flag, bit * are_equal(current[4], next[8]));
        result.agg_constraint(9, hash_init_flag, bit * are_equal(current[5], next[9]));
        result.agg_constraint(10, hash_init_flag, bit * are_equal(current[6], next[10]));
        result.agg_constraint(11, hash_init_flag, bit * are_equal(current[7], next[11]));

        // no additional constraints are imposed for nullifier hashing because we don't care what
        // happens to nullifier columns after step 7

        // enforce that values in the bit column must be binary
        result[24] = is_binary(current[24]);

        // finally, we need to make sure that at steps which are multiples of 8 (e.g. 0, 16, 32 etc.)
        // values in columns [4, 5, 6, 7] are the same as in columns [16, 17, 18, 19]; technically,
        // we care about this only for step 0, but it is easier to enforce it for all multiples of 8
        result.agg_constraint(25, key_cmp_flag, are_equal(current[4], current[16]));
        result.agg_constraint(26, key_cmp_flag, are_equal(current[5], current[17]));
        result.agg_constraint(27, key_cmp_flag, are_equal(current[6], current[18]));
        result.agg_constraint(28, key_cmp_flag, are_equal(current[7], current[19]));
    }

    // BOILERPLATE
    // --------------------------------------------------------------------------------------------

    fn context(&self) -> &AirContext<Felt> {
        &self.context
    }
}

// MASKS
// ================================================================================================
const HASH_CYCLE_MASK: [Felt; HASH_CYCLE_LEN] = [
    Felt::ONE,
    Felt::ONE,
    Felt::ONE,
    Felt::ONE,
    Felt::ONE,
    Felt::ONE,
    Felt::ONE,
    Felt::ZERO,
];

const KEY_CMP_MASK: [Felt; HASH_CYCLE_LEN] = [
    Felt::ONE,
    Felt::ZERO,
    Felt::ZERO,
    Felt::ZERO,
    Felt::ZERO,
    Felt::ZERO,
    Felt::ZERO,
    Felt::ZERO,
];
