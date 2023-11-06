use std::fs;
use std::io::{BufRead, BufReader, Error};

#[derive(Debug)]
pub struct Flags {
    print_line_numbers: bool,
    print_only_matching_filenames: bool,
    case_insensitive: bool,
    invert_matches: bool,
    matches_entire_lines: bool,
}

impl Flags {
    fn defaults() -> Flags {
        Flags {
            print_line_numbers: false,
            print_only_matching_filenames: false,
            case_insensitive: false,
            invert_matches: false,
            matches_entire_lines: false,
        }
    }

    pub fn new(flags: &[&str]) -> Self {
        let mut rv = Flags::defaults();
        for fl in flags {
            for ch in fl.chars() {
                match ch {
                    'n' => rv.print_line_numbers = true,
                    'l' => rv.print_only_matching_filenames = true,
                    'i' => rv.case_insensitive = true,
                    'v' => rv.invert_matches = true,
                    'x' => rv.matches_entire_lines = true,
                    _ => (),
                }
            }
        }
        rv
    }
}

pub fn grep(pattern: &str, flags: &Flags, files: &[&str]) -> Result<Vec<String>, Error> {
    let mut rv = vec![];
    let many_files = files.len() > 1;
    let pattern_lowercase = &String::from(pattern).to_lowercase()[..];
    for fname in files {
        let f = fs::File::open(fname)?;
        let reader = BufReader::new(f);

        for (n, line) in reader.lines().enumerate() {
            let line = line?;
            let mut line_matched = match (flags.case_insensitive, flags.matches_entire_lines) {
                (true, false) => line.to_lowercase().contains(pattern_lowercase),
                (false, false) => line.contains(pattern),
                (true, true) => line.to_lowercase() == pattern_lowercase,
                (false, true) => line == pattern,
            };
            if flags.invert_matches {
                line_matched = !line_matched;
            }
            if line_matched {
                if flags.print_only_matching_filenames {
                    rv.push(fname.to_string());
                    break;
                }
                let mut s = line.to_string();
                if flags.print_line_numbers {
                    s = format!("{}:{}", n + 1, s);
                }
                if many_files {
                    s = format!("{}:{}", fname, s);
                }
                rv.push(s);
            }
        }
    }
    Ok(rv)
}
