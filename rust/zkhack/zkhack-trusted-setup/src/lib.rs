pub mod data;

pub const PUZZLE_DESCRIPTION: &str = r#"Alice has computed a trusted setup for a Groth16 proof scheme.
She decided to use a 128-bit long secret, and she swears that she does not know the secret s needed to get this setup.
The trusted setup is constructed as follows using two additional scalars α and β:
* [s^i] G1 for 0 ⩽ i ⩽ 62,
* [α s^i] G1 for 0 ⩽ i ⩽ 31,
* [β s^i] G1 for 0 ⩽ i ⩽ 31,
* [s^i] G2 for 0 ⩽ i ⩽ 31.

Can you recover the secret anyway?"#;
