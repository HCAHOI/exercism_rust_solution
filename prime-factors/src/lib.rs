pub fn factors(n: u64) -> Vec<u64> {
    let mut n = n;
    let mut res = Vec::new();
    let mut cur = 2;

    while n > 1 {
        while n % cur == 0 {
            res.push(cur);
            n /= cur;
        }
        cur += 1;
    }
    res
}
