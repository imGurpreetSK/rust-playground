#[derive(Debug)]
pub struct HighScores {
    scores: Vec<u32>
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        HighScores { scores: scores.to_vec() }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores.as_slice()
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores.last().cloned()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.scores.iter().max().cloned()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut highest: i32 = -1;
        let mut middle = -1;
        let mut lowest = -1;
        
        self.scores.iter().for_each(|&score| {
            let score = score as i32;
            if score > highest {
                lowest = middle;
                middle = highest;
                highest = score;
            } else if score > middle && score > lowest {
                lowest = middle;
                middle = score;
            } else if score > lowest && score < middle {
                lowest = score;
            }
        });

        let mut result: Vec<u32> = Vec::new();
        if highest > -1 { result.push(highest as u32); }
        if middle > -1 { result.push(middle as u32); }
        if lowest > -1 { result.push(lowest as u32); }
        result
    }
}
