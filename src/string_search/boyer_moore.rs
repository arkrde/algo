use super::preprocess::preprocess_l;
use super::preprocess::preprocess_lprime;
use super::preprocess::preprocess_n;
use super::preprocess::preprocess_r;

pub struct Searcher {
    pat: String,
}

impl Searcher {
    pub fn from_str(text: &str) -> Searcher {
        Searcher {
            pat: text.to_string(),
        }
    }
    fn compute_good_suffix_distance(
        &self,
        prep_l: &preprocess_l::PrepL,
        prep_lprime: &preprocess_lprime::PrepLp,
        pos: usize,
    ) -> usize {
        let l_score = prep_l.score(pos);
        match l_score {
            Some(u) => u,
            _ => {
                let lp_score = prep_lprime.score(pos);
                match lp_score {
                    Some(v) => v,
                    _ => 0,
                }
            }
        }
    }
    pub fn search(&self, text: &str) -> Option<usize> {
        let prep_n = preprocess_n::PrepN::from_str(&self.pat);
        let prep_l = preprocess_l::PrepL::new(&prep_n);
        let prep_lprime = preprocess_lprime::PrepLp::new(&prep_n);
        let prep_r = preprocess_r::PrepR::from_str(&self.pat);

        let n: usize = self.pat.len();
        let m: usize = text.len();
        let mut k: usize = 0;
        while k < (m - n + 1) {
            let match_dist: usize = self
                .pat
                .chars()
                .rev()
                .zip(text[k..(k + n)].chars().rev())
                .take_while(|(a, b)| a == b)
                .count();
            match match_dist == n {
                true => return Some(k),
                false => match match_dist == 0 {
                    true => k = k + 1,
                    false => {
                        let mismatch_char = self.pat.chars().nth(n - 1 - match_dist).unwrap();
                        let bad_char_pos = prep_r.score(&mismatch_char).unwrap();
                        let bad_char_shift = match bad_char_pos < (n - 1 - match_dist) {
                            true => (n - 1 - match_dist - bad_char_pos),
                            false => 1,
                        };
                        let good_suffix_pos = self.compute_good_suffix_distance(
                            &prep_l,
                            &prep_lprime,
                            n - 1 - match_dist,
                        );
                        let good_suffix_shift = n - 1 - good_suffix_pos;
                        match bad_char_shift < good_suffix_shift {
                            true => k += good_suffix_shift,
                            _ => k += bad_char_shift,
                        }
                    }
                },
            }
        }
        None
    }
}

#[cfg(test)]
mod boyer_moore_tests {
    #[test]
    fn test_1() {
        let s = super::Searcher::from_str("abxyabxz");
        assert_eq!(s.search("xabxyabxyabxz"), Some(5));
        assert_eq!(s.search("xabxyabxyabx"), None);
        assert_eq!(s.search("xabxyabxyabxs"), None);
        assert_eq!(s.search("xabxybxyabxy"), None);
        assert_eq!(s.search("xabxycbxyabxy"), None);
    }
    #[test]
    fn test_2() {
        let s = super::Searcher::from_str("text");
        assert_eq!(s.search("this is a test text"), Some(15));
    }
}
