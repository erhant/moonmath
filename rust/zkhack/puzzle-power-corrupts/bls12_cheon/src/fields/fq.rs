use ark_ff::fields::{Fp384, MontBackend, MontConfig};

#[derive(MontConfig)]
#[modulus = "5739212180072054691886037078074598258234828766015465682035977006377650233269727206694786492328072119776769214299497"]
#[generator = "5"]
pub struct FqConfig;
pub type Fq = Fp384<MontBackend<FqConfig, 6>>;
