//! This is an implementation of a simple exact linear-time
//! string search algorithm using preprocess-Z
use crate::string_search::preprocess_z;
struct Searcher {
    pat: String,
    prep: preprocess_z::PrepZ,
}
impl Searcher {
    fn from_str(pat: &str) -> Searcher {
        Searcher {
            pat: pat.to_string(),
            prep: preprocess_z::PrepZ::from_str(pat),
        }
    }
    fn search(&self, text: &str) -> Option<usize> {
        let mut right: usize = 0;
        for (a, b) in text.chars().zip(self.pat.chars()) {
            match a == b {
                false => {
                    break;
                }
                true => {
                    right += 1;
                }
            }
        }
        match right == self.pat.len() {
            true => Some(0),
            false => {
                let mut left: usize = 0;
                let mut z: usize = 0;
                for k in 1..(self.pat.len() - 1) {
                    match k > right {
                        true => {
                            for (a, b) in text[k..].chars().zip(self.pat.chars()) {
                                match a == b {
                                    true => {
                                        z += 1;
                                    }
                                    false => {
                                        break;
                                    }
                                }
                            }
                            match z == self.pat.len() {
                                true => {
                                    return Some(k);
                                }
                                false => {
                                    left = k;
                                    right = z + k - 1;
                                }
                            }
                        }
                        false => {
                            let k1: usize = k - left;
                            let z1: usize = self.prep.score(k1).unwrap();
                            let beta: usize = right - k + 1;
                            match z1 < beta {
                                true => {
                                    z = z1;
                                    match z == self.pat.len() {
                                        true => {
                                            return Some(k);
                                        }
                                        false => {}
                                    }
                                }
                                false => {
                                    z = beta;
                                    for (a, b) in
                                        text[(right + 1)..].chars().zip(self.pat[z1..].chars())
                                    {
                                        match a == b {
                                            true => {
                                                z += 1;
                                            }
                                            false => {
                                                left = k;
                                                right = left + z;
                                                break;
                                            }
                                        }
                                    }
                                    match z == self.pat.len() {
                                        true => {
                                            return Some(k);
                                        }
                                        false => {}
                                    }
                                }
                            }
                        }
                    }
                }
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::string_search::linear_search;
    #[test]
    fn linear_search_test_1() {
        let s = linear_search::Searcher::from_str("abxyabxz");
        assert_eq!(s.search("xabxyabxyabxz"), Some(5));
        assert_eq!(s.search("xabxyabxyabx"), None);
        assert_eq!(s.search("xabxyabxyabxs"), None);
        assert_eq!(s.search("xabxybxyabxy"), None);
        assert_eq!(s.search("xabxycbxyabxy"), None);
    }
}
