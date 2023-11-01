use std::collections::HashMap;

pub fn plants(_diagram: &str, _student: &str) -> Vec<&'static str> {
    let name2idx = HashMap::from([
        ("Alice", 0),
        ("Bob", 1),
        ("Charlie", 2),
        ("David", 3),
        ("Eve", 4),
        ("Fred", 5),
        ("Ginny", 6),
        ("Harriet", 7),
        ("Ileana", 8),
        ("Joseph", 9),
        ("Kincaid", 10),
        ("Larry", 11),
    ]);

    let abbr2plant = HashMap::from([
        ('G', "grass"),
        ('C', "clover"),
        ('R', "radishes"),
        ('V', "violets"),
    ]);

    let plants = _diagram.split("\n").collect::<Vec<_>>();
    let idx = *name2idx.get(_student).unwrap();
    let mut res = vec![];

    for plant in plants {
        let c1 = plant.chars().nth(idx * 2).unwrap();
        let c2 = plant.chars().nth(idx * 2 + 1).unwrap();
        let p1 = *abbr2plant.get(&c1).unwrap();
        let p2 = *abbr2plant.get(&c2).unwrap();
        res.push(p1);
        res.push(p2);
    }

    res
}
