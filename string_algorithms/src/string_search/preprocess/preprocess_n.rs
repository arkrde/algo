use super::preprocess_z::PrepZ;

pub struct PrepN {
    // prep: preprocess_z::PrepZ,
    score_vec: Vec<usize>,
}

impl PrepN {
    pub fn from_str(text: &str) -> PrepN {
        let text_rev = text.chars().rev().collect::<String>();
        let prep_z = PrepZ::from_str(text_rev.as_str());
        let score_vec = (0..prep_z.len())
            .map(|idx| prep_z.score(prep_z.len() - 1 - idx).unwrap())
            .collect();
        PrepN { score_vec }
    }
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
