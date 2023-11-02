use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    let mut res = HashSet::new();

    for i in 1..sum / 2 {
        for j in (i + 1)..sum / 2 {
            let k = sum - i - j;
            if i * i + j * j == k * k {
                res.insert([i, j, k]);
            }
        }
    }

    res
}
