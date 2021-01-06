#[derive(Debug)]
pub struct HighScores<'a> {
    scores: &'a [u32],
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        HighScores { scores: scores }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        // What the heck?
        self.scores.last().copied()
        // let s = self.scores.into_iter().last()?;
        // Some(*s)
    }

    pub fn personal_best(&self) -> Option<u32> {
        // Hmmm...
        self.scores.iter().max().copied()
        // let best = self.scores.into_iter().max()?;
        // Some(*best)
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        // Waaaaaa this is nice!
        let mut v = self.scores.to_vec();
        v.sort_unstable_by(|a, b| b.cmp(a));
        v.truncate(3);
        v
        // let mut v = self.scores.clone().to_vec();
        // v.sort_by(|a, b| b.partial_cmp(a).unwrap());
        // v.into_iter().take(3).collect::<Vec<u32>>()
    }
}
