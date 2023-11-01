use rand::{thread_rng, Rng};

fn quick_pow(base: u64, exponent: u64, modulus: u64) -> u64 {
    let mut res = 1;
    let mut e = exponent;
    let mut b = base;

    while e > 0 {
        if e & 1 == 1 {
            res = (res * b) % modulus;
        }
        e >>= 1;
        b = (b * b) % modulus;
    }

    res
}

pub fn private_key(p: u64) -> u64 {
    let mut rng = thread_rng();
    rng.gen_range(2..p)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    quick_pow(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    quick_pow(b_pub, a, p)
}
