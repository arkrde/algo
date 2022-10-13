use std::collections;
pub struct PrepR {
    r: collections::HashMap<char, usize>,
}

impl PrepR {
    pub fn from_str(text: &str) -> PrepR {
        PrepR {
            r: text.chars().enumerate().map(|(pos, c)| (c, pos)).collect(),
        }
    }
    /// Getter method to provide R-score for any character `key` in a string `s`
    /// i.e. the position of the right-most occurance of `key` in `s`
    pub fn score(&self, key: &char) -> Option<usize> {
        match self.r.get(&key) {
            Some(u) => Some(*u),
            _ => None
        }
    }
}

#[cfg(test)]
mod preprocess_r_tests {
    #[test]
    fn preprocess_test() {
        let prep = super::PrepR::from_str("tpabxab");
        assert_eq!(prep.score(&'a'), Some(5));
        assert_eq!(prep.score(&'b'), Some(6));
        assert_eq!(prep.score(&'c'), None);
        assert_eq!(prep.score(&'d'), None);
        assert_eq!(prep.score(&'t'), Some(0));
    }
}
