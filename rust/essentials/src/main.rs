use num_bigint::{BigInt, RandBigInt};
use rand;

// TODO: move this to a module & test it from there
fn main() {
    // generate a large prime number and a generator
    let bit_size = 32;
    let p = BigInt::from(23u32);
    let g = BigInt::from(5u32);

    // generate a random secret number for Alice and Bob
    let mut rng = rand::thread_rng();
    let priv_a = rng.gen_bigint(bit_size);
    let priv_b = rng.gen_bigint(bit_size);

    // compute public values for Alice and Bob
    let pub_a = g.modpow(&priv_a, &p);
    let pub_b = g.modpow(&priv_b, &p);

    // compute the shared secret key
    let s1 = pub_b.modpow(&priv_a, &p);
    let s2 = pub_a.modpow(&priv_b, &p);

    // verify that both Alice and Bob have the same shared secret key
    assert_eq!(s1, s2);
}
