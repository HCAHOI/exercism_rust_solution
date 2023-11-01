use std::{collections::HashSet, str::FromStr};

#[derive(Debug, PartialEq, Eq)]
pub struct Dna {
    dna: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Rna {
    rna: String,
}

fn dna_check(c: char) -> bool {
    let valid = HashSet::from(['A', 'C', 'G', 'T']);
    valid.contains(&c)
}

fn rna_check(c: char) -> bool {
    let valid = HashSet::from(['A', 'C', 'G', 'U']);
    valid.contains(&c)
}

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        for (i, c) in dna.chars().enumerate() {
            if !dna_check(c) {
                return Err(i);
            }
        }

        Ok(Self {
            dna: String::from_str(dna).unwrap(),
        })
    }

    pub fn into_rna(self) -> Rna {
        let mut ns = vec![];
        for c in self.dna.chars() {
            let corr_c = match c {
                'A' => 'U',
                'C' => 'G',
                'T' => 'A',
                'G' => 'C',
                _ => panic!(),
            };
            ns.push(corr_c);
        }

        Rna::new(&String::from_iter(ns.iter())).unwrap()
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        for (i, c) in rna.chars().enumerate() {
            if !rna_check(c) {
                return Err(i);
            }
        }

        Ok(Self {
            rna: String::from_str(rna).unwrap(),
        })
    }
}
