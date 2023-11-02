use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let mut hands: BinaryHeap<_> = hands.iter().map(|&s| (PokerHand::parse(s), s)).collect();
    let (winning, s) = hands.pop().unwrap();
    let mut result = vec![s];
    while let Some((value, s)) = hands.pop() {
        if value < winning {
            break;
        }
        result.push(s);
    }
    result
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct PokerHand {
    counts: Vec<usize>,
    values: Vec<u8>,
}

fn parse_card(s: &str) -> (u8, u8) {
    let (value, suit) = s.split_at(s.len() - 1);
    (
        match value.parse::<u8>() {
            Ok(v) => v,
            Err(_) => "JQKA".find(value).unwrap() as u8 + 11,
        },
        suit.as_bytes()[0],
    )
}

impl PokerHand {
    fn parse(s: &str) -> Self {
        let (values, suits): (Vec<u8>, Vec<u8>) = s.split_whitespace().map(parse_card).unzip();
        let mut groups = HashMap::<u8, usize>::new();
        for &v in values.iter() {
            *groups.entry(v).or_default() += 1;
        }
        let mut groups: Vec<_> = groups.into_iter().map(|(v, c)| (c, v)).collect();
        groups.sort_unstable_by_key(|&x| Reverse(x));
        let (mut counts, mut values): (Vec<_>, Vec<_>) = groups.iter().copied().unzip();
        if counts.len() == 5 {
            if values == [14, 5, 4, 3, 2] {
                values = vec![5, 4, 3, 2, 1];
            }
            let is_straight = values[0] - values[4] == 4;
            let is_flush = suits[1..].iter().all(|&x| x == suits[0]);
            match (is_straight, is_flush) {
                (true, true) => counts = vec![5],
                (true, false) => counts = vec![3, 1, 2],
                (false, true) => counts = vec![3, 1, 3],
                _ => {}
            }
        }
        Self { counts, values }
    }
}
