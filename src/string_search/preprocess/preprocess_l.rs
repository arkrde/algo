use super::preprocess_n;

pub struct PrepL<'a> {
    prep_n: &'a preprocess_n::PrepN,
    l: Vec<usize>,
}

impl<'a> PrepL<'a> {
    pub fn new(p: &'a preprocess_n::PrepN) -> PrepL {
        let l = vec![p.len(); p.len()];
        let mut prep = PrepL { prep_n: p, l: l };
        prep.compute();
        prep
    }
    fn compute(&mut self) {
        for j in 0..(self.prep_n.len() - 1) {
            let n_score = self.prep_n.score(j).unwrap();
            match n_score > 0 {
                true => {
                    let i = self.prep_n.len() - n_score;
                    self.l[i] = j;
                }
                false => {}
            }
        }
    }
    pub fn score(&self, idx: usize) -> Option<usize> {
        match idx < self.l.len() {
            false => None,
            true => match self.l[idx] < self.l.len() {
                false => None,
                true => Some(self.l[idx]),
            },
        }
    }
}

#[cfg(test)]
mod preprocess_l_tests {
    #[test]
    fn preprocess_test() {
        let prep_n = super::preprocess_n::PrepN::from_str("cabdabdab");
        let prep = super::PrepL::new(&prep_n);
        assert_eq!(prep.score(0), None);
        assert_eq!(prep.score(1), None);
        assert_eq!(prep.score(2), None);
        assert_eq!(prep.score(3), None);
        assert_eq!(prep.score(4), Some(5));
        assert_eq!(prep.score(7), Some(2));
        assert_eq!(prep.score(8), None);
    }

    #[test]
    fn preprocess_test_unique_chars() {
        let prep_n = super::preprocess_n::PrepN::from_str("abcdefgh");
        let prep = super::PrepL::new(&prep_n);
        for i in 0..8 {
            assert_eq!(prep.score(i), None);
        }
    }

    #[test]
    fn preprocess_test_repeat_chars() {
        let prep_n = super::preprocess_n::PrepN::from_str("aaaaaaaa");
        let prep = super::PrepL::new(&prep_n);
        assert_eq!(prep.score(0), None);
        assert_eq!(prep.score(1), Some(6));
        assert_eq!(prep.score(2), Some(5));
        assert_eq!(prep.score(3), Some(4));
        assert_eq!(prep.score(4), Some(3));
        assert_eq!(prep.score(5), Some(2));
        assert_eq!(prep.score(6), Some(1));
        assert_eq!(prep.score(7), Some(0));
    }

    #[test]
    fn preprocess_test_repeat_patterns() {
        let prep_n = super::preprocess_n::PrepN::from_str("abababab");
        let prep = super::PrepL::new(&prep_n);
        assert_eq!(prep.score(0), None);
        assert_eq!(prep.score(1), None);
        assert_eq!(prep.score(2), Some(5));
        assert_eq!(prep.score(3), None);
        assert_eq!(prep.score(4), Some(3));
        assert_eq!(prep.score(5), None);
        assert_eq!(prep.score(6), Some(1));
        assert_eq!(prep.score(7), None);
    }
}
