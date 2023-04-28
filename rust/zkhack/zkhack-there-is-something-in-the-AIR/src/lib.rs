//! This module implements a simple version of Semaphore protocol using STARK proving system.
//!
//! The Semaphore protocol has the following properties:
//! - There is a set of users each possessing a (secret) private key.
//! - Each user can derive a public key from their private key.
//! - A collection of public keys forms an "access set".
//!
//! Once users have formed an access set, they should be able to do the following:
//! - A user should be able to send a signal on a topic (a topic could be any string).
//! - Each signal should have a unique identifier. This identifier is called a nullifier.
//!   Thus, a combination (private key, topic) should always produce the same nullifier.
//!   This ensures that the same private key cannot be used to signal more than once on
//!   a given topic.
//! - Given a signal, it should not be possible to infer which user made it. That is,
//!   signaling must be private.
//!
//! In our concrete setting, the protocol is implemented as follows:
//! - A secret key is represented by 4 field elements in a 64-bit prime field. This means that
//!   a secret key is roughly 256-bits long.
//! - A public key is derived by hashing private key with zero - i.e., hash(priv_key, 0). For
//!   the hash function we use Rp64_256 (which is an instantiation of Rescue Prime). The output
//!   of this hash function (the digest) is also 4 field elements in the same field.
//! - An access set is built by combining public keys into a Merkle tree using the same Rp64_256
//!   hash function.
//! - A nullifier is computed by hashing a private key together with a hash of the topic - i.e.:
//!   hash(priv_key, hash(topic)) using the same Rp64_256 hash function.
//! - To make a signal on a topic, the user first computes a nullifier as described above, and then
//!   generates a STARK proof attesting that the nullifier was computed correctly, and that the
//!   user's private key is present in the Merkle tree representing a given access set. Thus,
//!   a signal is a tuple consisting of (nullifier, STARK proof).
//! - Given a topic, a root of a Merkle tree representing an access set, and a signal, anyone
//!   can verify that this signal is indeed a valid signal against this topic by verifying
//!   the STARK proof.

use std::{convert::TryInto, fmt, ops::Range};
use winter_utils::{Deserializable, SliceReader};
use winterfell::{
    crypto::{hashers::Rp64_256 as Rescue, Digest as HasherDigest, Hasher, MerkleTree},
    math::{fields::f64::BaseElement as Felt, log2, FieldElement, StarkField},
    ProofOptions, Prover, StarkProof, Trace, TraceTable,
};

mod air;
use air::{PublicInputs, SemaphoreAir};

mod prover;
use prover::SemaphoreProver;

// TYPE ALIASES
// ================================================================================================

/// Output type for Rescue Prime hash function, which consists of four field elements in a 64-bit
/// prime field.
type Digest = <Rescue as Hasher>::Digest;

// ACCESS SET
// ================================================================================================

/// Defines an access set of users who can signal on a given topic.
///
/// An access set is a collection of users' public keys which internally are stored in a Merkle
/// tree. For simplicity, we impose the following restrictions on the size of an access set:
/// - The number of public keys must be exactly a power of two.
/// - Base two logarithm of the number of public keys must be one less than a power of two.
///
/// Thus, for example, the following set sizes are allowed: 8, 128, 32768 etc.
pub struct AccessSet(MerkleTree<Rescue>);

impl AccessSet {
    pub fn new(pub_keys: Vec<PubKey>) -> Self {
        // make sure we got a valid number of public keys
        let num_keys = pub_keys.len();
        assert!(
            num_keys.is_power_of_two(),
            "number of public keys must be a power of two"
        );
        assert!(
            (log2(num_keys) + 1).is_power_of_two(),
            "base two logarithm of the number of public keys must be one less than a power of two"
        );

        // convert public keys into leaves of a Merkle tree; we do this simply by extracting
        // public keys inner type (which is a Digest) from each key.
        let leaves = pub_keys.iter().map(|p| p.0).collect::<Vec<_>>();

        // build a Merkle tree from the public key leaves
        let key_tree = MerkleTree::new(leaves).unwrap();
        Self(key_tree)
    }

    /// Returns the root of this access set.
    pub fn root(&self) -> Digest {
        *self.0.root()
    }

    /// Returns a Merkle path to the key at the specified index.
    ///
    /// The first element in the path will be the public key itself.
    pub fn get_key_path(&self, key_idx: usize) -> Vec<Digest> {
        self.0.prove(key_idx).expect("invalid key index")
    }

    /// Returns a signal of the user with specified private key on the specified topic.
    ///
    /// The signal includes a unique nullifier for the combination of (priv_key, topic), as well
    /// as the proof that the public key for the provided private key exists in this access set.
    pub fn make_signal(&self, priv_key: &PrivKey, topic: &str) -> Signal {
        // compute hash of the topic
        let topic = Rescue::hash(topic.as_bytes());

        // get the index of the key in the Merkle tree
        let pub_key = PubKey::new(priv_key);
        let key_idx = self
            .0
            .leaves()
            .iter()
            .position(|&v| v == pub_key.0)
            .expect("public key for the provided private key could not be found");

        // get the path to the key from the Merkle tree
        let key_path = self
            .0
            .prove(key_idx)
            .expect("failed to build a Merkle path for key index");

        // compute the nullifier for this key and topic
        let nullifier = priv_key.get_nullifier(topic);

        // build the proof asserting that the key is in the access set and that if hashed with
        // the specified topic it produces a given nullifier.
        let prover = SemaphoreProver::default();
        let trace = prover.build_trace(priv_key, key_idx, topic, &key_path);
        let proof = prover.prove(trace).expect("failed to generate proof");

        // return the signal
        Signal { nullifier, proof }
    }

    /// Returns Ok(()) if the provided signal is a valid signal on the specified topic by someone
    /// with a key from this access set.
    pub fn verify_signal(&self, topic: &str, signal: Signal) -> Result<(), String> {
        // create public inputs for proof verification
        let pub_inputs = PublicInputs {
            tree_root: self.root(),
            nullifier: signal.nullifier,
            topic: Rescue::hash(topic.as_bytes()),
        };

        // check if the STARK proof is valid against the above public inputs
        match winterfell::verify::<SemaphoreAir>(signal.proof, pub_inputs) {
            Ok(_) => Ok(()),
            Err(err) => Err(format!("proof verification failed: {}", err)),
        }
    }
}

// SIGNAL
// ================================================================================================

/// Defines a signal on a topic.
///
/// A signal contains an `nullifier`, which is a unique value for a combination of (private key,
/// topic). The signal also contains a STARK proof which attests that a signal was made by someone
/// with a private key which was used to compute the nullifier, and that this private key belongs
/// to a given access set.
#[derive(Debug, Clone)]
pub struct Signal {
    pub nullifier: Digest,
    pub proof: StarkProof,
}

impl fmt::Display for Signal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Nullifier: {}", hex::encode(self.nullifier.as_bytes()))?;
        writeln!(
            f,
            "Proof size: {:.1} KB",
            self.proof.to_bytes().len() as f64 / 1024f64
        )?;
        write!(
            f,
            "Proof security: {} bits",
            self.proof.security_level(true)
        )
    }
}

// PUBLIC KEY
// ================================================================================================

/// Defines a public key for a given private key.
///
/// A public key is simply a hash of a private key.
#[derive(Debug)]
pub struct PubKey(Digest);

impl PubKey {
    /// Returns a [PubKey] instantiated from the provided private key.
    ///
    /// The key is computed simply as hash(priv_key, 0).
    pub fn new(priv_key: &PrivKey) -> Self {
        let priv_key_elements: [Felt; 4] = priv_key.elements();
        let priv_key_hash = Rescue::merge(&[priv_key_elements.into(), [Felt::ZERO; 4].into()]);
        Self(priv_key_hash)
    }

    /// Returns a [PubKey] parsed from the provided string.
    ///
    /// # Panics
    /// Panics if the string does not represent a valid encoding of a public key.
    pub fn parse(key: &str) -> Self {
        let key_bytes = hex::decode(key).expect("invalid key encoding");
        let mut key_reader = SliceReader::new(&key_bytes);
        let key_elements = Felt::read_batch_from(&mut key_reader, 4).expect("invalid key encoding");
        let key_array: [Felt; 4] = key_elements
            .try_into()
            .expect("failed to convert vector to array");
        Self(key_array.into())
    }

    /// Returns elements which make up this public key.
    pub fn elements(&self) -> [Felt; 4] {
        self.0.into()
    }
}

impl fmt::Display for PubKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", hex::encode(self.0.as_bytes()))
    }
}

// PRIVATE KEY
// ================================================================================================

/// Defines a private key of a user.
///
/// A private key is simply 4 field elements in a 64-bit prime field, which means that a key is
/// roughly 256 bits in size.
#[derive(Debug)]
pub struct PrivKey([Felt; 4]);

impl PrivKey {
    /// Returns a [PrivKey] instantiated with the provided elements.
    pub fn new(elements: [Felt; 4]) -> Self {
        Self(elements)
    }

    /// Returns a [PrivKey] parsed from the provided string.
    ///
    /// # Panics
    /// Panics if the string does not represent a valid encoding of a private key.
    pub fn parse(key: &str) -> Self {
        let key_bytes = hex::decode(key).expect("invalid key encoding");
        let mut key_reader = SliceReader::new(&key_bytes);
        let key_elements = Felt::read_batch_from(&mut key_reader, 4).expect("invalid key encoding");
        Self(
            key_elements
                .try_into()
                .expect("failed to convert vector to array"),
        )
    }

    /// Creates a nullifier for the provided topic against this private key.
    ///
    /// A nullifier is computed simply as hash(key, topic).
    pub fn get_nullifier(&self, topic: Digest) -> Digest {
        let key: Digest = self.0.into();
        Rescue::merge(&[key, topic])
    }

    /// Returns elements which make up this private key.
    pub fn elements(&self) -> [Felt; 4] {
        self.0
    }
}

impl fmt::Display for PrivKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", hex::encode(Felt::elements_as_bytes(&self.0)))
    }
}

// CONSTANTS
// ================================================================================================

/// Number of rounds for Rescue Prime hash function; this is equal to 7.
const NUM_HASH_ROUNDS: usize = Rescue::NUM_ROUNDS;

/// Number of the execution trace rows needed to compute Rescue Prime permutation; this is equal
/// to 8.
const HASH_CYCLE_LEN: usize = NUM_HASH_ROUNDS.next_power_of_two();

/// Number of columns in the execution trace.
const TRACE_WIDTH: usize = 25;

// UTILITIES
// ================================================================================================

/// Prints out an execution trace.
pub fn print_trace(
    trace: &TraceTable<Felt>,
    multiples_of: usize,
    offset: usize,
    range: Range<usize>,
) {
    let trace_width = trace.width();

    let mut state = vec![Felt::ZERO; trace_width];
    for i in 0..trace.length() {
        if (i.wrapping_sub(offset)) % multiples_of != 0 {
            continue;
        }
        trace.read_row_into(i, &mut state);
        println!(
            "{}\t{:?}",
            i,
            state[range.clone()]
                .iter()
                .map(|v| v.as_int())
                .collect::<Vec<u64>>()
        );
    }
}
