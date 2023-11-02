use std::{cmp::Ordering, collections::HashMap, fmt::Display, str::FromStr};
const NAME_LEN: usize = 31;

#[derive(Eq, PartialEq, Clone)]
struct Team {
    name: String,
    matches: usize,
    wins: usize,
    draws: usize,
    loses: usize,
}

impl Team {
    fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            matches: 0,
            wins: 0,
            draws: 0,
            loses: 0,
        }
    }

    fn add_win(&mut self) {
        self.matches += 1;
        self.wins += 1;
    }

    fn add_lose(&mut self) {
        self.matches += 1;
        self.loses += 1;
    }

    fn add_draw(&mut self) {
        self.matches += 1;
        self.draws += 1;
    }

    fn score(&self) -> usize {
        self.wins * 3 + self.draws
    }
}

impl Display for Team {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // name
        let mut name = self.name.clone();
        // add whitespace
        name += &" ".repeat(NAME_LEN - self.name.len());
        write!(
            f,
            "{}| {:2} | {:2} | {:2} | {:2} | {:2}",
            name,
            self.matches,
            self.wins,
            self.draws,
            self.loses,
            self.score()
        )
    }
}

pub fn tally(match_results: &str) -> String {
    let header =
        String::from_str("Team                           | MP |  W |  D |  L |  P").unwrap();

    let results = match_results.split("\n").collect::<Vec<_>>();
    if results.len() == 1 && results[0].len() < 1 {
        return header;
    }
    let mut teams_m: HashMap<&str, Team> = HashMap::new();
    for result in results {
        let words = result.split(";").collect::<Vec<_>>();
        let (mut t1, mut t2) = (Team::new(words[0]), Team::new(words[1]));
        if teams_m.contains_key(words[0]) {
            t1 = teams_m.get(words[0]).unwrap().clone();
        }
        if teams_m.contains_key(words[1]) {
            t2 = teams_m.get(words[1]).unwrap().clone();
        }
        match words[2] {
            "win" => {
                t1.add_win();
                t2.add_lose();
            }
            "draw" => {
                t1.add_draw();
                t2.add_draw();
            }
            "loss" => {
                t1.add_lose();
                t2.add_win();
            }
            _ => panic!(),
        };
        teams_m.insert(words[0], t1);
        teams_m.insert(words[1], t2);
    }
    let mut teams_v = teams_m.iter().map(|(_, v)| v).collect::<Vec<_>>();
    teams_v.sort_by(|a, b| match a.score().cmp(&b.score()) {
        Ordering::Less => Ordering::Greater,
        Ordering::Greater => Ordering::Less,
        Ordering::Equal => a.name.cmp(&b.name),
    });
    let mut res = format!("{}\n", header);
    for (idx, t) in teams_v.iter().enumerate() {
        res += &format!("{}", t);
        if idx != teams_v.len() - 1 {
            res += "\n";
        }
    }
    println!("{}", res);
    res
}
