use super::preprocess_n;

pub struct PrepLp {
    score_vec: Vec<usize>,
}

impl PrepLp {
    pub fn new(p: &preprocess_n::PrepN) -> PrepLp {
        let score_vec = vec![0; p.len()];
        let mut prep = PrepLp { score_vec };
        prep.compute(&p);
        prep
    }
    fn compute(&mut self, p: &preprocess_n::PrepN) {
        let mut val: usize = 0;
        for j in 0..p.len() {
            match p.score(j) == Some(j + 1) {
                true => {
                    self.score_vec[j] = j + 1;
                    val = j + 1;
                }
                false => self.score_vec[j] = val,
            }
        }
    }
    pub fn score(&self, idx: usize) -> Option<usize> {
        match idx < self.score_vec.len() {
            false => None,
            true => Some(self.score_vec[self.score_vec.len() - 1 - idx]),
        }
    }
}

#[cfg(test)]
mod preprocess_lp_tests {
    #[test]
    fn preprocess_test() {
        let prep_n = super::preprocess_n::PrepN::from_str("cabdabdab");
        let prep = super::PrepLp::new(&prep_n);
        assert_eq!(prep.score(0), Some(9));
        assert_eq!(prep.score(1), Some(0));
        assert_eq!(prep.score(2), Some(0));
        assert_eq!(prep.score(3), Some(0));
        assert_eq!(prep.score(4), Some(0));
        assert_eq!(prep.score(7), Some(0));
        assert_eq!(prep.score(8), Some(0));
    }

    #[test]
    fn preprocess_test_unique_chars() {
        let prep_n = super::preprocess_n::PrepN::from_str("abcdefgh");
        let prep = super::PrepLp::new(&prep_n);
        assert_eq!(prep.score(0), Some(8));
        assert_eq!(prep.score(1), Some(0));
        assert_eq!(prep.score(2), Some(0));
        assert_eq!(prep.score(3), Some(0));
        assert_eq!(prep.score(4), Some(0));
        assert_eq!(prep.score(5), Some(0));
        assert_eq!(prep.score(6), Some(0));
        assert_eq!(prep.score(7), Some(0));
    }

    #[test]
    fn preprocess_test_repeat_chars() {
        let prep_n = super::preprocess_n::PrepN::from_str("aaaaaaaa");
        let prep = super::PrepLp::new(&prep_n);
        assert_eq!(prep.score(0), Some(8));
        assert_eq!(prep.score(1), Some(7));
        assert_eq!(prep.score(2), Some(6));
        assert_eq!(prep.score(3), Some(5));
        assert_eq!(prep.score(4), Some(4));
        assert_eq!(prep.score(5), Some(3));
        assert_eq!(prep.score(6), Some(2));
        assert_eq!(prep.score(7), Some(1));
    }

    #[test]
    fn preprocess_test_repeat_patterns() {
        let prep_n = super::preprocess_n::PrepN::from_str("abababab");
        let prep = super::PrepLp::new(&prep_n);
        assert_eq!(prep.score(0), Some(8));
        assert_eq!(prep.score(1), Some(6));
        assert_eq!(prep.score(2), Some(6));
        assert_eq!(prep.score(3), Some(4));
        assert_eq!(prep.score(4), Some(4));
        assert_eq!(prep.score(5), Some(2));
        assert_eq!(prep.score(6), Some(2));
        assert_eq!(prep.score(7), Some(0));
    }
}
