pub struct PrepZ {
    s: String,
    z: Vec<usize>,
    left: usize,
    right: usize,
}

impl PrepZ {
    /// Constructs the preprocessor from an string slice
    pub fn from_str(text: &str) -> PrepZ {
        let mut p = PrepZ {
            s: text.to_string(),
            z: vec![text.len(); text.len()],
            left: 0,
            right: 0,
        };
        p.compute();
        p
    }
    /// Compute the Z-value for each position of the string
    fn compute(&mut self) {
        for i in 1..self.s.len() {
            self.compute_impl(i);
        }
    }
    /// Helper method for `compute`
    fn compute_impl(&mut self, k: usize) {
        match k > self.right {
            true => {
                self.z[k] = self.s[k..]
                    .chars()
                    .zip(self.s.chars())
                    .take_while(|(a, b)| a == b)
                    .count();
                if self.z[k] > 0 {
                    self.left = k;
                    self.right = k + self.z[k] - 1;
                }
            }
            false => {
                let k1 = k - self.left;
                let beta: usize = self.right - k + 1;
                match self.z[k1] < beta.try_into().unwrap() {
                    true => {
                        self.z[k] = self.z[k1];
                    }
                    false => {
                        self.right += self.s[(self.right + 1)..]
                            .chars()
                            .zip(self.s[(beta + 1)..].chars())
                            .take_while(|(a, b)| a == b)
                            .count();
                        self.left = k;
                        self.z[k] = self.right - k + 1;
                    }
                }
            }
        }
    }
    /// Getter method to provide Z-score for each position of a string
    /// i.e. the length of the longest substring of `s` that starts at `idx`
    /// and is a prefix of `s`
    pub fn score(&self, idx: usize) -> Option<usize> {
        match idx < self.z.len() {
            true => Some(self.z[idx]),
            _ => None,
        }
    }
    pub fn len(&self) -> usize {
        self.z.len()
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
        assert_eq!(s.left, 9);
        assert_eq!(s.right, 15);
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
