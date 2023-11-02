use lazy_static::lazy_static;
use rand::{thread_rng, Rng};
use std::{collections::HashSet, sync::Mutex};

lazy_static! {
    static ref NAME_DB: Mutex<HashSet<String>> = Mutex::new(HashSet::new());
}
pub struct Robot {
    name: String,
}

fn gen_name() -> String {
    let mut rng = thread_rng();
    let mut cs = vec![];
    for _ in 0..2 {
        cs.push(rng.gen_range('A'..='Z'));
    }
    for _ in 0..3 {
        cs.push(rng.gen_range('0'..='9'));
    }
    String::from_iter(cs.iter())
}

fn gen_robot_name() -> String {
    let mut new_name = String::new();
    let mut db = NAME_DB.lock().unwrap();
    loop {
        new_name = gen_name();
        if !db.contains(&new_name) {
            break;
        }
    }
    db.insert(new_name.clone());
    new_name
}

impl Robot {
    pub fn new() -> Self {
        let name = gen_robot_name();
        Self { name: name }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn reset_name(&mut self) {
        let new_name = gen_robot_name();
        self.name = new_name;
    }
}
