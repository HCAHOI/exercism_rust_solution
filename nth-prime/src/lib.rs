fn is_prime(n: u32) -> bool {
    (2..=(n as f64).sqrt().round() as u32).fold(0, |acc, x| if n % x == 0 { acc + 1 } else { acc })
        == 0
}

pub fn nth(n: u32) -> u32 {
    let mut cnt = 0;
    let mut cur = 2;
    loop {
        if is_prime(cur) {
            if cnt == n {
                break;
            }
            cnt += 1;
        }
        cur += 1;
    }
    cur
}
