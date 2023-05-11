pub mod bls;
pub mod data;
pub mod hash;

pub const PUZZLE_DESCRIPTION: &str = r#"Alice designed an authentication system in which users gain access by presenting it a signature on a username, which Alice provided.
One day, Alice discovered 256 of these signatures were leaked publicly, but the secret key wasn't. Phew.
The next day, she found out someone accessed her system with a username she doesn't know! This shouldn't be possible due to existential unforgeability, as she never signed such a message.

Can you find out how it happend and produce a signature on your username?"#;
