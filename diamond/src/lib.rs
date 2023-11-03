fn get_idx(c: char) -> usize {
    (c as u8 - 'A' as u8) as usize
}

pub fn get_diamond(c: char) -> Vec<String> {
    let size = get_idx(c) * 2 + 1;
    let chs =
        ('A'..c).into_iter().collect::<String>() + &('A'..=c).rev().into_iter().collect::<String>();
    chs.chars()
        .map(|ch| {
            let idx = get_idx(ch);
            let mut res = String::new();
            if idx == 0 {
                res += &" ".repeat((size - 1) / 2);
                res += &ch.to_string();
                res += &" ".repeat((size - 1) / 2);
            } else {
                res += &" ".repeat((size - idx * 2) / 2);
                res += &ch.to_string();
                res += &" ".repeat(idx * 2 - 1);
                res += &ch.to_string();
                res += &" ".repeat((size - idx * 2) / 2);
            }
            res
        })
        .collect::<Vec<_>>()
}
