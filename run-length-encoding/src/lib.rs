pub fn encode(source: &str) -> String {
    let digits = source.chars().collect::<Vec<_>>();
    let mut cnt = 0;
    let mut last = char::MAX;
    let mut res = String::new();
    for c in digits {
        if last != c {
            if last != char::MAX {
                if cnt != 1 {
                    res += &cnt.to_string()[..]
                };
                res += &last.to_string()[..];
            }
            last = c;
            cnt = 0;
        }
        cnt += 1;
    }

    if last != char::MAX {
        if cnt != 1 {
            res += &cnt.to_string()[..]
        };
        res += &last.to_string()[..];
    }

    res
}

pub fn decode(source: &str) -> String {
    let mut res = String::new();
    let digits = source.chars().collect::<Vec<_>>();
    let mut cnt = 0;
    for c in digits {
        if c.is_digit(10) {
            cnt = cnt * 10 + c.to_digit(10).unwrap();
            continue;
        }
        if cnt == 0 {
            cnt = 1;
        }
        res += &c.to_string()[..].repeat(cnt as usize);
        cnt = 0;
    }
    res
}
