pub struct PrepZ {
    score_vec: Vec<usize>,
    left_pos: usize,
    right_pos: usize,
}

impl PrepZ {
    /// Constructs the preprocessor from an string slice
    pub fn from_str(text: &str) -> PrepZ {
        let mut prep = PrepZ {
            // pat: text,
            score_vec: vec![text.len(); text.len()],
            left_pos: 0,
            right_pos: 0,
        };
        prep.compute(text);
        prep
    }
    /// Compute the Z-value for each position of the string
    fn compute(&mut self, text: &str) {
        for i in 1..text.len() {
            self.compute_impl(text, i);
        }
    }
    /// Helper method for `compute`
    fn compute_impl(&mut self, text: &str, k: usize) {
        match k > self.right_pos {
            true => {
                self.score_vec[k] = text[k..]
                    .chars()
                    .zip(text.chars())
                    .take_while(|(a, b)| a == b)
                    .count();
                if self.score_vec[k] > 0 {
                    self.left_pos = k;
                    self.right_pos = k + self.score_vec[k] - 1;
                }
            }
            false => {
                let k1 = k - self.left_pos;
                let beta: usize = self.right_pos - k + 1;
                match self.score_vec[k1] < beta {
                    true => {
                        self.score_vec[k] = self.score_vec[k1];
                    }
                    false => {
                        self.right_pos += text[(self.right_pos + 1)..]
                            .chars()
                            .zip(text[(beta + 1)..].chars())
                            .take_while(|(a, b)| a == b)
                            .count();
                        self.left_pos = k;
                        self.score_vec[k] = self.right_pos - k + 1;
                    }
                }
            }
        }
    }
    /// Getter method to provide Z-score for each position of a string
    /// i.e. the length of the longest substring of `s` that starts at `idx`
    /// and is a prefix of `s`
    pub fn score(&self, idx: usize) -> Option<usize> {
        match idx < self.score_vec.len() {
            true => Some(self.score_vec[idx]),
            _ => None,
        }
    }
    pub fn len(&self) -> usize {
        self.score_vec.len()
    }
}

#[cfg(test)]
mod preprocess_z_tests {
    #[test]
    fn preprocess_test_1() {
        let s = super::PrepZ::from_str("aabcaabxaaz");
        assert_eq!(s.len(), 11);
        assert_eq!(s.score(0), Some(11));
        assert_eq!(s.score(1), Some(1));
        assert_eq!(s.score(2), Some(0));
        assert_eq!(s.score(3), Some(0));
        assert_eq!(s.score(4), Some(3));
        assert_eq!(s.score(5), Some(1));
        assert_eq!(s.score(6), Some(0));
        assert_eq!(s.score(7), Some(0));
        assert_eq!(s.score(8), Some(2));
        assert_eq!(s.score(9), Some(1));
        assert_eq!(s.score(10), Some(0));
    }
    #[test]
    fn preprocess_test_2() {
        let s = super::PrepZ::from_str("aabaabcaxaabaabcy");
        assert_eq!(s.len(), 17);
        assert_eq!(s.score(9), Some(7));
        assert_eq!(s.left_pos, 9);
        assert_eq!(s.right_pos, 15);
    }

    #[test]
    fn preprocess_test_3() {
        let s = super::PrepZ::from_str("abxyabxz");
        assert_eq!(s.len(), 8);
        assert_eq!(s.score(0), Some(8));
        assert_eq!(s.score(1), Some(0));
        assert_eq!(s.score(2), Some(0));
        assert_eq!(s.score(3), Some(0));
        assert_eq!(s.score(4), Some(3));
        assert_eq!(s.score(5), Some(0));
        assert_eq!(s.score(6), Some(0));
        assert_eq!(s.score(7), Some(0));
    }

    #[test]
    fn preprocess_test_4() {
        let s = super::PrepZ::from_str("aaaaaaaa");
        assert_eq!(s.len(), 8);
        assert_eq!(s.score(0), Some(8));
        assert_eq!(s.score(1), Some(7));
        assert_eq!(s.score(2), Some(6));
        assert_eq!(s.score(3), Some(5));
        assert_eq!(s.score(4), Some(4));
        assert_eq!(s.score(5), Some(3));
        assert_eq!(s.score(6), Some(2));
        assert_eq!(s.score(7), Some(1));
    }
}
