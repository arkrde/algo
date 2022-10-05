use crate::string_search::preprocess_z;

pub struct PrepN {
    prep: preprocess_z::PrepZ,
}

impl PrepN {
    pub fn from_str(text: &str) -> PrepN {
        let text_rev = text.chars().rev().collect::<String>();
        PrepN {
            prep: preprocess_z::PrepZ::from_str(&text_rev),
        }
    }
    pub fn score(&self, idx: usize) -> Option<usize> {
        let idx = self.prep.len() - 1 - idx;
        self.prep.score(idx)
    }
    pub fn len(&self) -> usize {
        self.prep.len()
    }
}

#[cfg(test)]
mod preprocess_n_tests {
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
        assert_eq!(s.score(8), Some(9));
    }

    #[test]
    fn preprocess_test_2() {
        let s = super::PrepN::from_str("abcabcab");
        assert_eq!(s.score(0), Some(0));
        assert_eq!(s.score(1), Some(2));
        assert_eq!(s.score(2), Some(0));
        assert_eq!(s.score(3), Some(0));
        assert_eq!(s.score(4), Some(5));
        assert_eq!(s.score(5), Some(0));
        assert_eq!(s.score(6), Some(0));
        assert_eq!(s.score(7), Some(8));
    }

    #[test]
    fn preprocess_test_3() {
        let s = super::PrepN::from_str("abcdefgh");
        assert_eq!(s.score(0), Some(0));
        assert_eq!(s.score(1), Some(0));
        assert_eq!(s.score(2), Some(0));
        assert_eq!(s.score(3), Some(0));
        assert_eq!(s.score(4), Some(0));
        assert_eq!(s.score(5), Some(0));
        assert_eq!(s.score(6), Some(0));
        assert_eq!(s.score(7), Some(8));
    }

    #[test]
    fn preprocess_test_4() {
        let s = super::PrepN::from_str("abcdefga");
        assert_eq!(s.score(0), Some(1));
        assert_eq!(s.score(1), Some(0));
        assert_eq!(s.score(2), Some(0));
        assert_eq!(s.score(3), Some(0));
        assert_eq!(s.score(4), Some(0));
        assert_eq!(s.score(5), Some(0));
        assert_eq!(s.score(6), Some(0));
        assert_eq!(s.score(7), Some(8));
    }

    #[test]
    fn preprocess_test_5() {
        let s = super::PrepN::from_str("aaaaaaaa");
        assert_eq!(s.score(0), Some(1));
        assert_eq!(s.score(1), Some(2));
        assert_eq!(s.score(2), Some(3));
        assert_eq!(s.score(3), Some(4));
        assert_eq!(s.score(4), Some(5));
        assert_eq!(s.score(5), Some(6));
        assert_eq!(s.score(6), Some(7));
        assert_eq!(s.score(7), Some(8));
    }
}
