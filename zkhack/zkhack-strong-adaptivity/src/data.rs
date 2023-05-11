use ark_serialize::CanonicalDeserialize;

use crate::msg_equality_arg::CommitKey;

const COMMIT_KEY_BYTES: &'static str = "nQh/RnHH1d/QHhlcux36Gn6Zz8nAxt5k32RawD+Mi5lefDDjKVZ53fjfz0DyWQezBA0CxUDNJnAXF+iskdSSyw==";

pub fn puzzle_data() -> CommitKey {
    let ck = CommitKey::deserialize(&*base64::decode(COMMIT_KEY_BYTES).unwrap()).unwrap();
    ck
}
