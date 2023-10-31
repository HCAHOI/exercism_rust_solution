#[derive(Debug)]
pub struct HighScores {
    scores: Vec<u32>,
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        Self {
            scores: Vec::from(scores),
        }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores.as_ref()
    }

    pub fn latest(&self) -> Option<u32> {
        if self.scores.is_empty() {
            None
        } else {
            Some(self.scores.last().unwrap().clone())
        }
    }

    pub fn personal_best(&self) -> Option<u32> {
        let mut sorted = self.scores.clone();
        sorted.sort_unstable();
        match sorted.last() {
            Some(v) => Some(v.clone()),
            None => None,
        }
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut sorted = self.scores.clone();
        sorted.sort_unstable();
        let mut res = vec![];
        for _ in 0..3 {
            if let Some(v) = sorted.pop() {
                res.push(v);
            };
        }
        res
    }
}
