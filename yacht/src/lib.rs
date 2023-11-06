pub enum Category {
    Ones,
    Twos,
    Threes,
    Fours,
    Fives,
    Sixes,
    FullHouse,
    FourOfAKind,
    LittleStraight,
    BigStraight,
    Choice,
    Yacht,
}

type Dice = [u8; 5];

pub fn score(dice: Dice, category: Category) -> u8 {
    use Category::*;
    let mut dice = dice;
    dice.sort();
    match (category, dice) {
        (Ones, _) => dice.into_iter().filter(|d| *d == 1).sum(),
        (Twos, _) => dice.into_iter().filter(|d| *d == 2).sum(),
        (Threes, _) => dice.into_iter().filter(|d| *d == 3).sum(),
        (Fours, _) => dice.into_iter().filter(|d| *d == 4).sum(),
        (Fives, _) => dice.into_iter().filter(|d| *d == 5).sum(),
        (Sixes, _) => dice.into_iter().filter(|d| *d == 6).sum(),
        (FullHouse, [a, b, c, d, e]) if a == b && (b == c || c == d) && d == e && a != e => {
            dice.into_iter().sum()
        }
        (FourOfAKind, [a, b, c, d, _] | [_, a, b, c, d]) if (a == b && b == c && c == d) => {
            a + b + c + d
        }
        (LittleStraight, [1, 2, 3, 4, 5]) => 30,
        (BigStraight, [2, 3, 4, 5, 6]) => 30,
        (Choice, _) => dice.into_iter().sum(),
        (Yacht, [5, 5, 5, 5, 5]) => 50,
        _ => 0,
    }
}