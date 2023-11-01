pub fn collatz(n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }
    return compute(n, 0);
}

fn compute(n: u64, cnt: u64) -> Option<u64> {
    if n == 1 {
        return Some(cnt);
    }
    if n % 2 == 0 {
        if let Some(v) = compute(n / 2, cnt) {
            return Some(v + 1);
        } else {
            None
        }
    } else {
        if let Some(v1) = n.checked_mul(3) {
            if let Some(v2) = v1.checked_add(1) {
                if let Some(v) = compute(v2, cnt) {
                    return Some(v + 1);
                } else {
                    return None;
                }
            } else {
                return None;
            }
        } else {
            return None;
        }
    }
}
