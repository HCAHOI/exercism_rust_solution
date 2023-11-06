// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

use std::collections::{BTreeMap, HashMap};

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidRowCount(usize),
    InvalidColumnCount(usize),
}

pub fn convert(input: &str) -> Result<String, Error> {
    let line_num = input.lines().count();
    if line_num % 4 != 0 {
        return Err(Error::InvalidRowCount(line_num));
    }
    for line in input.lines() {
        let col_num = line.chars().count();
        if col_num % 3 != 0 {
            return Err(Error::InvalidColumnCount(col_num));
        }
    }

    let lines = input.lines().collect::<Vec<_>>();

    let mut res = vec![];

    for single_digit_lines in lines.chunks(4) {
        let mut digit_line_cache = BTreeMap::new();
        for line in single_digit_lines {
            let line_chars = line.chars().collect::<Vec<_>>();

            for (char_idx, char_cols) in line_chars.chunks(3).enumerate() {
                let char_chars = digit_line_cache.entry(char_idx).or_insert(vec![]);
                for c in char_cols {
                    char_chars.push(*c);
                }
            }
        }

        let mut single_digits = vec![];
        for (_, v) in digit_line_cache {
            single_digits.push(convert_character(&v));
        }

        res.push(String::from_iter(single_digits.iter()));
    }

    Ok(res.join(","))
}

fn convert_character(input: &Vec<char>) -> char {
    if &input[..] == [' ', '_', ' ', '|', ' ', '|', '|', '_', '|', ' ', ' ', ' '] {
        '0'
    } else if &input[..] == [' ', ' ', ' ', ' ', ' ', '|', ' ', ' ', '|', ' ', ' ', ' '] {
        '1'
    } else if &input[..] == [' ', '_', ' ', ' ', '_', '|', '|', '_', ' ', ' ', ' ', ' '] {
        '2'
    } else if &input[..] == [' ', '_', ' ', ' ', '_', '|', ' ', '_', '|', ' ', ' ', ' '] {
        '3'
    } else if &input[..] == [' ', ' ', ' ', '|', '_', '|', ' ', ' ', '|', ' ', ' ', ' '] {
        '4'
    } else if &input[..] == [' ', '_', ' ', '|', '_', ' ', ' ', '_', '|', ' ', ' ', ' '] {
        '5'
    } else if &input[..] == [' ', '_', ' ', '|', '_', ' ', '|', '_', '|', ' ', ' ', ' '] {
        '6'
    } else if &input[..] == [' ', '_', ' ', ' ', ' ', '|', ' ', ' ', '|', ' ', ' ', ' '] {
        '7'
    } else if &input[..] == [' ', '_', ' ', '|', '_', '|', '|', '_', '|', ' ', ' ', ' '] {
        '8'
    } else if &input[..] == [' ', '_', ' ', '|', '_', '|', ' ', '_', '|', ' ', ' ', ' '] {
        '9'
    } else {
        '?'
    }
}
