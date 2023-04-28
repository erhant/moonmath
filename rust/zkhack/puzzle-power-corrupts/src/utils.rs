use ark_bls12_cheon::Fr;
use ark_ff::biginteger::BigInteger128;
use ark_ff::{Field, One};

use num_traits::pow;
use std::collections::HashMap;

// Calculates p ^ exp
// bitlen: bit length of exp, cant be more than 126
pub fn pow_sp<S: Field>(p: S, exp: u128, bitlen: u32) -> S {
    let mut res = S::one();
    // iterate over all bits in exp, in Big endian order.
    for b in (128 - bitlen - 2)..128 {
        res = res * (res);
        let bi = exp & (1 << (127 - b)) > 0;
        // println!("index: {}, next bit: {}, exp: {}", b, bi, exp);
        if bi {
            res = res * p;
        }
    }
    return res;
}

// returns p^exp, p^(exp^2), ...., p^(exp^(n))
// assumes exp is 80bits:
pub fn pow_sp2<S: Field>(p: S, exp: Fr, n: u64) -> HashMap<S, u64> {
    let m = 80 as u32;
    let k = 16 as u32;
    let mut vec = Vec::with_capacity(n as usize);
    let mut map = HashMap::new();

    // populate lookup table:
    // 2^k sized array for 80/k buckets
    let num_bkt: u32 = (m / k);
    let p2k: u64 = pow(2, k as usize);

    // let mut lookup_table: [ [S; 2^k]; num_bkt as usize];
    let mut lookup_table: Vec<Vec<S>> = Vec::new();
    // All buckets should take ~equal time to fill.
    for bi in 0..num_bkt {
        let mut bkt_i: Vec<S> = Vec::new();
        bkt_i.push(S::one()); // p^(0*2^(k*bi))
        if bi == 0 {
            bkt_i.push(p);
        } else {
            bkt_i.push(pow_sp(
                lookup_table[(bi - 1) as usize][1],
                p2k as u128,
                k + 1 as u32,
            )); // lookup_table[bi-1][1] ^ (2^k)
        }
        for j in 2..p2k {
            bkt_i.push(bkt_i[(j - 1) as usize] * bkt_i[1]);
        }
        lookup_table.push(bkt_i);
    }

    // raising
    let mut pows_fr: Vec<Fr> = Vec::with_capacity(n as usize);
    let mut pows: Vec<u128> = Vec::with_capacity(n as usize);
    pows_fr.push(Fr::from(1)); // exp^0
    pows.push(1);
    for i in 1..n {
        pows_fr.push(pows_fr[(i - 1) as usize] * exp);
        pows.push(bigInt_to_u128(pows_fr[i as usize].into()));
    }

    let mut count = 0;
    // [For testing] for pi in rnd_pows.iter() // pow_start..pow_end
    for pi in pows.iter() {
        let mut gpi = S::one();
        for bi in 0..num_bkt {
            // ind = pi & ( (p2k-1) << (27*bi) )
            let ind = ((*pi) >> (k * bi)) & (p2k as u128 - 1);
            // get lookup[bi][ind]
            gpi = gpi * lookup_table[bi as usize][ind as usize];
        }
        vec.push(gpi);
        map.insert(gpi, count);
        count = count + 1;
    }
    return map;
}

pub fn bigInt_to_u128(bi: BigInteger128) -> u128 {
    return (bi.0[0] as u128 + pow(2 as u128, 64) * (bi.0[1] as u128)) as u128;
}
