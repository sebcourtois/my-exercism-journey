#[derive(Debug)]
pub struct HighScores<'a> {
    raw_scores: &'a [u32],
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        Self { raw_scores: scores }
    }

    pub fn scores(&self) -> &[u32] {
        self.raw_scores
    }

    pub fn latest(&self) -> Option<u32> {
        self.raw_scores.last().cloned()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.raw_scores.iter().max().cloned()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut scores = self.raw_scores.to_vec();
        scores.sort();
        scores.reverse();
        scores.truncate(3);
        scores
    }
}
