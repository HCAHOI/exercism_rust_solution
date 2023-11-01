use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

// This annotation prevents Clippy from warning us that `School` has a
// `fn new()` with no arguments, but doesn't implement the `Default` trait.
//
// Normally, it's good practice to just do what Clippy tells you, but in this
// case, we want to keep things relatively simple. The `Default` trait is not the point
// of this exercise.
#[allow(clippy::new_without_default)]
pub struct School {
    stu: HashMap<String, u32>,
}

impl School {
    pub fn new() -> School {
        Self {
            stu: HashMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        self.stu.insert(String::from_str(student).unwrap(), grade);
    }

    pub fn grades(&self) -> Vec<u32> {
        let grades_set = self.stu.iter().map(|(_, &v)| v).collect::<HashSet<_>>();
        let mut grades = grades_set.iter().map(|&v| v).collect::<Vec<_>>();
        grades.sort_unstable();
        grades
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {
        let mut res = self
            .stu
            .iter()
            .filter_map(|(k, v)| if v == &grade { Some(k.clone()) } else { None })
            .collect::<Vec<String>>();
        res.sort();
        res
    }
}
