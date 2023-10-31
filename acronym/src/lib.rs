use std::str::FromStr;

fn split_camel_case(input: &str) -> Vec<String> {
    let mut substrings = Vec::new();
    let mut current_word = String::new();
    for c in input.chars() {
        if c.is_uppercase() && !current_word.is_empty() {
            substrings.push(current_word);
            current_word = String::new();
        }
        current_word.push(c);
    }
    if !current_word.is_empty() {
        substrings.push(current_word);
    }
    substrings
}

fn check_is_camal(str: &str) -> bool {
    let mut low = false;
    let mut high = false;
    for c in str.chars() {
        if c.is_uppercase() {
            high = true;
        } else {
            low = true;
        }
    }
    high && low
}

pub fn abbreviate(phrase: &str) -> String {
    // split & convert
    let delimiters = [' ', '-'];
    let phrase = String::from_str(phrase).unwrap();
    let splited = phrase
        .split(|c| delimiters.contains(&c))
        .into_iter()
        .collect::<Vec<_>>();

    let mut words = vec![];

    for word in splited {
        println!("{} {}", word, check_is_camal(word));
        if check_is_camal(word) {
            let temp = split_camel_case(word);
            for tmp in temp {
                words.push(tmp);
            }
        } else {
            words.push(String::from_str(word).unwrap());
        }
    }

    String::from_iter(
        words
            .iter()
            .map(|word| word.to_uppercase()) //to string
            .map(|word| String::from_str(word.trim_matches(|c: char| !c.is_alphabetic())).unwrap()) //trim
            .filter_map(|word| word.chars().nth(0)) //get first char
            .collect::<Vec<_>>()
            .iter(),
    )
}
