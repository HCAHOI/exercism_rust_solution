pub fn number(user_number: &str) -> Option<String> {
    let mut cnt = 0;
    let mut res = vec![];
    for c in user_number.chars().rev() {
        if !(c.is_digit(10) || c == '(' || c == ')' || c == '-' || c.is_whitespace() || c == '.') {
            return None;
        }

        if c.is_digit(10) {
            let v = String::from(c).parse::<u64>().unwrap();
            if cnt == 6 || cnt == 9 {
                if v < 2 {
                    return None;
                }
            }

            if cnt == 10 && v != 1 {
                return None;
            } else if cnt == 10 {
                break;
            }

            cnt += 1;
            res.push(v.to_string());
        }
    }

    if cnt != 10 {
        return None;
    }

    res.reverse();
    Some(res.join(""))
}
