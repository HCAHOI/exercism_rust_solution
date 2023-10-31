use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut set = HashSet::new();

    for &f in factors {
        if f == 0 {
            continue;
        }
        let mut multiplier = 2;
        let mut x = f;
        while x < limit {
            set.insert(x);
            x = f * multiplier;
            multiplier += 1;
        }
    }

    set.iter().sum()
}
