use std::collections::HashSet;

fn is_prime(n: u64) -> bool {
    for i in 2..(n as f64).sqrt().round() as u64 {
        if n % i == 0 {
            return false;
        }
    }
    true
}

pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut non_prime = HashSet::new();
    let mut res = vec![];

    for i in 2..=upper_bound {
        if !non_prime.contains(&i) {
            if is_prime(i) {
                res.push(i);
                let mut factor = 2;
                while i * factor <= upper_bound {
                    non_prime.insert(factor * i);
                    factor += 1;
                }
            }
        }
    }

    res.sort_unstable();
    res
}
