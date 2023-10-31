/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let bar_pos = [11, 5, 1];
    let mut isbn = isbn.chars().collect::<Vec<_>>();
    // length check
    if isbn.len() == 13 {
        println!("length check pass");
        // bar check
        for idx in bar_pos {
            if isbn[idx] != '-' {
                return false;
            }
        }

        println!("bar check pass");
        // char check1
        for idx in 0..11 {
            if idx != 1 && idx != 5 && !isbn[idx].is_numeric() {
                return false;
            }
        }

        isbn.remove(11);
        isbn.remove(5);
        isbn.remove(1);
    }

    if isbn.len() != 10 {
        return false;
    }

    println!("char check1 pass");
    //char check2
    println!("{}", isbn[9].is_numeric());
    if !isbn[9].is_numeric() && isbn[9] != 'X' {
        return false;
    }

    println!("char check2 pass");
    let nums = isbn
        .iter()
        .filter(|&c| c.is_numeric() || c.clone() == 'X')
        .map(|&c| {
            if c.is_numeric() {
                c.to_digit(10).unwrap()
            } else {
                10
            }
        })
        .collect::<Vec<_>>();

    let mut cnt = 11;
    nums.iter().fold(0, |acc, x| {
        cnt -= 1;
        acc + cnt * x
    }) % 11
        == 0
}
