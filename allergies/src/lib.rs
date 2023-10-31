pub struct Allergies {
    allergic: Vec<u32>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

impl Allergies {
    fn enum2idx(e: &Allergen) -> usize {
        match e {
            Allergen::Eggs => 0,
            Allergen::Peanuts => 1,
            Allergen::Shellfish => 2,
            Allergen::Strawberries => 3,
            Allergen::Tomatoes => 4,
            Allergen::Chocolate => 5,
            Allergen::Pollen => 6,
            Allergen::Cats => 7,
        }
    }

    fn idx2enum(idx: usize) -> Allergen {
        match idx {
            0 => Allergen::Eggs,
            1 => Allergen::Peanuts,
            2 => Allergen::Shellfish,
            3 => Allergen::Strawberries,
            4 => Allergen::Tomatoes,
            5 => Allergen::Chocolate,
            6 => Allergen::Pollen,
            7 => Allergen::Cats,
            _ => panic!(),
        }
    }

    pub fn new(score: u32) -> Self {
        let mut score = score;
        let mut res = Self {
            allergic: vec![0; 8],
        };
        let mut cnt = 0;
        while score > 0 && cnt < 8 {
            res.allergic[cnt] = score % 2;
            score /= 2;
            cnt += 1;
        }
        println!("{:?}", res.allergic);
        res
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.allergic[Self::enum2idx(allergen)] == 1
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        let mut res = vec![];
        for idx in 0..8 {
            if self.allergic[idx] == 1 {
                res.push(Self::idx2enum(idx));
            }
        }
        res
    }
}
