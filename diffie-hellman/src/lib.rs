use num::{BigUint, ToPrimitive};
use rand::{thread_rng, Rng};

pub fn private_key(p: u64) -> u64 {
    let mut rng = thread_rng();
    rng.gen_range(2..p)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    let p = BigUint::from(p);
    let g = BigUint::from(g);
    let a = BigUint::from(a);
    g.modpow(&a, &p).to_u64().unwrap()
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    let p = BigUint::from(p);
    let b_pub = BigUint::from(b_pub);
    let a = BigUint::from(a);
    b_pub.modpow(&a, &p).to_u64().unwrap()
}
