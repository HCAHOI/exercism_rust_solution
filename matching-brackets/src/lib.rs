use std::collections::VecDeque;

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack = VecDeque::new();
    for c in string.chars() {
        match c {
            '{' | '[' | '(' => stack.push_back(c),
            '}' => {
                if stack.pop_back() != Some('{') {
                    return false;
                }
            }
            ']' => {
                if stack.pop_back() != Some('[') {
                    return false;
                }
            }
            ')' => {
                if stack.pop_back() != Some('(') {
                    return false;
                }
            }
            _ => (),
        }
    }

    stack.is_empty()
}
