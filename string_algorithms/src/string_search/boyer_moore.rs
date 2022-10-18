use super::preprocess::preprocess_l::PrepL;
use super::preprocess::preprocess_lprime::PrepLp;
use super::preprocess::preprocess_n::PrepN;
use super::preprocess::preprocess_r::PrepR;

pub struct BoyerMooreSearcher<'a> {
    pat: &'a str,
    text: &'a str,
    pos: usize,
    prep_l: PrepL,
    prep_lprime: PrepLp,
    prep_r: PrepR,
}

impl<'a> BoyerMooreSearcher<'a> {
    pub fn new(pat: &'a str, text: &'a str) -> BoyerMooreSearcher<'a> {
        let pos: usize = 0;
        let prep_n = PrepN::from_str(&pat);
        let prep_l = PrepL::new(&prep_n);
        let prep_lprime = PrepLp::new(&prep_n);
        let prep_r = PrepR::from_str(&pat);
        BoyerMooreSearcher {
            pat,
            text,
            pos,
            prep_l,
            prep_lprime,
            prep_r,
        }
    }
    fn compute_good_suffix_distance(
        prep_l: &PrepL,
        prep_lprime: &PrepLp,
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
    fn search_helper(
        pat: &str,
        text: &str,
        start_pos: usize,
        prep_l: &PrepL,
        prep_lprime: &PrepLp,
        prep_r: &PrepR,
    ) -> Option<(usize, usize)> {
        let n: usize = pat.len();
        let m: usize = text.len();
        let mut k: usize = start_pos;
        while k < (m - n + 1) {
            let match_dist: usize = pat
                .chars()
                .rev()
                .zip(text[k..(k + n)].chars().rev())
                .take_while(|(a, b)| a == b)
                .count();
            match match_dist == n {
                true => return Some((k, k + n - prep_lprime.score(1).unwrap_or(n - 1))),
                false => match match_dist == 0 {
                    true => k = k + 1,
                    false => {
                        let mismatch_char = pat.chars().nth(n - 1 - match_dist).unwrap();
                        let bad_char_pos = prep_r.score(&mismatch_char).unwrap();
                        let bad_char_shift = match bad_char_pos < (n - 1 - match_dist) {
                            true => (n - 1 - match_dist - bad_char_pos),
                            false => 1,
                        };
                        let good_suffix_pos = Self::compute_good_suffix_distance(
                            &prep_l,
                            &prep_lprime,
                            n - 1 - match_dist,
                        );
                        let good_suffix_shift = match good_suffix_pos < n {
                            true => n - 1 - good_suffix_pos,
                            _ => 1,
                        };
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

impl<'a> Iterator for BoyerMooreSearcher<'a> {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        if self.pos > (self.text.len() - self.pat.len()) {
            None
        } else {
            match Self::search_helper(
                self.pat,
                self.text,
                self.pos,
                &self.prep_l,
                &self.prep_lprime,
                &self.prep_r,
            ) {
                Some((u, v)) => {
                    self.pos = v;
                    Some(u)
                }
                None => None,
            }
        }
    }
}

#[cfg(test)]
mod string_algorithm_tests {
    #[test]
    fn boyer_mooore_test() {
        let pat = "abxyabxz";
        let text = "xabxyabxyabxz";
        let mut s = super::BoyerMooreSearcher::new(pat, text);
        assert_eq!(s.next(), Some(5));
        let results: Vec<usize> = super::BoyerMooreSearcher::new("ab", "xabxyabxyabxz").collect();
        assert_eq!(results, vec![1, 5, 9]);
        assert_eq!(
            super::BoyerMooreSearcher::new("abxyabxz", "xabxyabxyabxz").collect::<Vec<usize>>(),
            vec![5]
        );
        assert_eq!(
            super::BoyerMooreSearcher::new("abxyabxz", "xabxyabxyabx").collect::<Vec<usize>>(),
            vec![]
        );
        assert_eq!(
            super::BoyerMooreSearcher::new("abxyabxz", "xabxyabxyabxs").collect::<Vec<usize>>(),
            vec![]
        );
        assert_eq!(
            super::BoyerMooreSearcher::new("abxyabxz", "xabxybxyabxy").collect::<Vec<usize>>(),
            vec![]
        );
        assert_eq!(
            super::BoyerMooreSearcher::new("abxyabxz", "xabxycbxyabxy").collect::<Vec<usize>>(),
            vec![]
        );
    }
    #[test]
    fn boyer_mooore_test_2() {
        let text =
            "TTGAATGCTGAAATCAGCAGGTAATATATGATAATAGAGAAAGCTATCCCGAAGGTGCATAGGTCAACAATACTTGAGCC";
        let pat = "TGA";
        let searcher = super::BoyerMooreSearcher::new(pat, text);
        let results: Vec<usize> = searcher.collect();
        println!("{:?}", results);
    }
    #[test]
    fn boyer_mooore_test_3() {
        let text = "MDSKGSSQKGSRLLLLLVVSNLLLCQGVVSTPVCPNGPGNCQVSLRDLFDRAVMVSHYIHDLSS\
            EMFNEFDKRYAQGKGFITMALNSCHTSSLPTPEDKEQAQQTHHEVLMSLILGLLRSWNDPLYHL\
            VTEVRGMKGAPDAILSRAIEIEEENKRLLEGMEMIFGQVIPGAKETEPYPVWSGLPSLQTKDED\
            ARYSAFYNLLHCLRRDSSKIDTYLKLLNCRIIYNNNC";
        let pat = "LLLLLVVSN";
        let searcher = super::BoyerMooreSearcher::new(pat, text);
        let results: Vec<usize> = searcher.collect();
        println!("{:?}", results);
    }
}
