pub struct RailFence {
    rails: u32,
}

enum Dir {
    Down,
    Up,
}

impl RailFence {
    pub fn new(rails: u32) -> RailFence {
        Self { rails }
    }

    pub fn encode(&self, text: &str) -> String {
        let mut rails = vec![vec!['.'; text.len()]; self.rails as usize];
        let mut rail = 0;
        let mut idx = 0;
        let mut d = Dir::Down;
        for c in text.chars() {
            rails[rail][idx] = c;
            match d {
                Dir::Down => {
                    if rail == rails.len() - 1 {
                        rail -= 1;
                        d = Dir::Up;
                    } else {
                        rail += 1;
                    }
                }
                Dir::Up => {
                    if rail == 0 {
                        rail += 1;
                        d = Dir::Down;
                    } else {
                        rail -= 1;
                    }
                }
            };
            idx += 1;
        }
        for v in &rails {
            println!("{:?}", v);
        }
        let mut res = String::new();
        for v in rails {
            for c in v {
                if c.is_alphabetic() {
                    res.push(c);
                }
            }
        }
        res
    }

    pub fn decode(&self, cipher: &str) -> String {
        let mut rails = vec![vec!['.'; cipher.len()]; self.rails as usize];
        let cipher = cipher.chars().collect::<Vec<_>>();
        let mut rail = 0;
        let mut idx = 0;
        let mut d = Dir::Down;
        for _ in 0..cipher.len() {
            rails[rail][idx] = 'x';
            match d {
                Dir::Down => {
                    if rail == rails.len() - 1 {
                        rail -= 1;
                        d = Dir::Up;
                    } else {
                        rail += 1;
                    }
                }
                Dir::Up => {
                    if rail == 0 {
                        rail += 1;
                        d = Dir::Down;
                    } else {
                        rail -= 1;
                    }
                }
            };
            idx += 1;
        }

        let mut cnt = 0;
        for v in &mut rails {
            for c in v {
                if c.is_alphabetic() {
                    *c = cipher[cnt];
                    cnt += 1;
                }
            }
        }

        let mut rail = 0;
        let mut idx = 0;
        let mut d = Dir::Down;
        let mut res = String::new();
        for _ in 0..cipher.len() {
            res.push(rails[rail][idx]);
            match d {
                Dir::Down => {
                    if rail == rails.len() - 1 {
                        rail -= 1;
                        d = Dir::Up;
                    } else {
                        rail += 1;
                    }
                }
                Dir::Up => {
                    if rail == 0 {
                        rail += 1;
                        d = Dir::Down;
                    } else {
                        rail -= 1;
                    }
                }
            };
            idx += 1;
        }

        res
    }
}
