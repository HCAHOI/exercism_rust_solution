use std::collections::{HashMap, VecDeque};

pub type Value = i32;
pub type Result = std::result::Result<(), Error>;

pub struct Forth {
    symbol_map: HashMap<String, Vec<String>>,
    value_stack: Vec<Value>,
}

#[derive(Debug, PartialEq, Eq)]
enum Mode {
    Eval,
    DefineSymbol,
    DefineValue,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

impl Forth {
    pub fn new() -> Forth {
        Self {
            symbol_map: HashMap::new(),
            value_stack: Vec::new(),
        }
    }

    pub fn stack(&self) -> &[Value] {
        self.value_stack.as_slice()
    }

    pub fn eval(&mut self, input: &str) -> Result {
        let mut mode = Mode::Eval;
        let mut eval_stack = VecDeque::new();
        let mut symbol_def = ("".to_string(), Vec::new());
        for token in input.split_ascii_whitespace() {
            eval_stack.push_back(token.to_string());
            while let Some(ref elem) = eval_stack.pop_front() {
                let elem = elem.to_lowercase();
                match elem.as_str() {
                    ":" => mode = Mode::DefineSymbol,
                    ";" => {
                        mode = Mode::Eval;
                        if symbol_def.0 == "" {
                            return Err(Error::InvalidWord);
                        }
                        self.symbol_map
                            .insert(symbol_def.0.clone(), symbol_def.1.clone());
                        symbol_def = ("".to_string(), Vec::new());
                    }
                    "+" | "-" | "*" | "/" => match mode {
                        Mode::DefineSymbol => {
                            mode = Mode::DefineValue;
                            symbol_def.0 = elem.to_string();
                        }
                        Mode::DefineValue => symbol_def.1.push(elem.to_string()),
                        Mode::Eval => match self.symbol_map.get(&elem) {
                            Some(expansion) => {
                                eval_stack.extend(expansion.iter().map(String::to_string))
                            }
                            None => {
                                let (right_op, left_op) = (
                                    self.value_stack.pop().ok_or(Error::StackUnderflow)?,
                                    self.value_stack.pop().ok_or(Error::StackUnderflow)?,
                                );
                                let val = match elem.as_str() {
                                    "*" => left_op * right_op,
                                    "+" => left_op + right_op,
                                    "-" => left_op - right_op,
                                    "/" => {
                                        if right_op == 0 {
                                            return Err(Error::DivisionByZero);
                                        } else {
                                            left_op / right_op
                                        }
                                    }
                                    _ => unreachable!(),
                                };
                                self.value_stack.push(val);
                            }
                        },
                    },
                    num if num.parse::<i32>().is_ok() => match mode {
                        Mode::DefineSymbol => return Err(Error::InvalidWord),
                        Mode::DefineValue => {
                            symbol_def.1.push(elem.to_string());
                        }
                        Mode::Eval => self.value_stack.push(num.parse().unwrap()),
                    },
                    word => match mode {
                        Mode::DefineSymbol => {
                            mode = Mode::DefineValue;
                            symbol_def.0 = word.to_string();
                        }
                        Mode::DefineValue => match self.symbol_map.get(word) {
                            Some(expansion) => {
                                if word == symbol_def.0 {
                                    eval_stack.extend(expansion.iter().map(String::to_string))
                                } else {
                                    let key = format!("{}_{}", word, symbol_def.0);
                                    self.symbol_map.insert(key.clone(), expansion.clone());
                                    symbol_def.1.push(key);
                                }
                            }
                            None => match word {
                                "dup" | "swap" | "drop" | "over" => {
                                    symbol_def.1.push(word.to_string())
                                }
                                _ => return Err(Error::UnknownWord),
                            },
                        },
                        Mode::Eval => match self.symbol_map.get(word) {
                            None => match word {
                                "dup" => {
                                    let prev_elem =
                                        self.value_stack.pop().ok_or(Error::StackUnderflow)?;
                                    self.value_stack.push(prev_elem);
                                    self.value_stack.push(prev_elem);
                                }
                                "swap" => {
                                    let (first, second) = (
                                        self.value_stack.pop().ok_or(Error::StackUnderflow)?,
                                        self.value_stack.pop().ok_or(Error::StackUnderflow)?,
                                    );
                                    self.value_stack.push(first);
                                    self.value_stack.push(second);
                                }
                                "drop" => {
                                    self.value_stack.pop().ok_or(Error::StackUnderflow)?;
                                }
                                "over" => {
                                    let (first, second) = (
                                        self.value_stack.pop().ok_or(Error::StackUnderflow)?,
                                        self.value_stack.pop().ok_or(Error::StackUnderflow)?,
                                    );
                                    self.value_stack.push(second);
                                    self.value_stack.push(first);
                                    self.value_stack.push(second);
                                }
                                _ => return Err(Error::UnknownWord),
                            },
                            Some(expansion) => {
                                eval_stack.extend(expansion.iter().map(String::to_string))
                            }
                        },
                    },
                }
            }
        }
        match mode {
            Mode::DefineSymbol | Mode::DefineValue => Err(Error::InvalidWord),
            _ => Ok(()),
        }
    }
}
