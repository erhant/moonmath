use ark_ff::fields::{Fp128, MontBackend, MontConfig};

#[derive(MontConfig)]
#[modulus = "1114157594638178892192613"]
#[generator = "2"]
pub struct FrConfig;
pub type Fr = Fp128<MontBackend<FrConfig, 2>>;
