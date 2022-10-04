use crate::string_search::preprocess_z;

struct PrepN {
    prep: preprocess_z::PrepZ,
}

impl PrepN {
    fn from_str(text: &str) -> PrepN {
        let text_rev = text.chars().rev().collect::<String>();
        PrepN {
            prep: preprocess_z::PrepZ::from_str(&text_rev),
        }
    }
    fn score(&self, idx: usize) -> Option<usize> {
        let idx = self.prep.len() - 1 - idx;
        self.prep.score(idx)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn preprocess_test() {
        let s = super::PrepN::from_str("cabdabdab");
        assert_eq!(s.score(0), Some(0));
        assert_eq!(s.score(1), Some(0));
        assert_eq!(s.score(2), Some(2));
        assert_eq!(s.score(3), Some(0));
        assert_eq!(s.score(4), Some(0));
        assert_eq!(s.score(5), Some(5));
        assert_eq!(s.score(6), Some(0));
        assert_eq!(s.score(7), Some(0));
    }
}
