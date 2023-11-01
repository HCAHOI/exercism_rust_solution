use std::{collections::VecDeque, str::FromStr};

pub fn answer(command: &str) -> Option<i32> {
    let mut words = command
        .split(" ")
        .enumerate()
        .filter_map(|(i, v)| {
            if i.clone() > 1 && v != "by" {
                Some(String::from_str(v).unwrap())
            } else {
                None
            }
        })
        .collect::<VecDeque<_>>();

    if let Some(mut s) = words.pop_back() {
        s.remove(s.len() - 1);
        words.push_back(s);
    }

    println!("{:?}", words);

    while words.len() > 2 {
        if let Ok(v1) = words.pop_front().unwrap().parse::<i32>() {
            let op = words.pop_front().unwrap();
            if let Ok(v2) = words.pop_front().unwrap().parse::<i32>() {
                let mut t_res = 0;
                match &op[..] {
                    "plus" => t_res = v1 + v2,
                    "minus" => t_res = v1 - v2,
                    "multiplied" => t_res = v1 * v2,
                    "divided" => t_res = v1 / v2,
                    _ => return None,
                }
                words.push_front(t_res.to_string());
            } else {
                return None;
            }
        } else {
            return None;
        }
    }

    if words.len() == 1 {
        Some(words.pop_front().unwrap().parse::<i32>().unwrap())
    } else {
        None
    }
}
