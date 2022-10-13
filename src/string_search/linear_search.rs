//! This is an implementation of a simple exact linear-time
//! string search algorithm using preprocess-Z
use super::preprocess::preprocess_z;
pub struct Searcher {
    pat: String,
    prep: preprocess_z::PrepZ,
}
impl Searcher {
    pub fn from_str(pat: &str) -> Searcher {
        Searcher {
            pat: pat.to_string(),
            prep: preprocess_z::PrepZ::from_str(pat),
        }
    }
    pub fn search(&self, text: &str) -> Option<usize> {
        let mut right: usize = text
            .chars()
            .zip(self.pat.chars())
            .take_while(|(a, b)| a == b)
            .count();
        match right == self.pat.len() {
            true => Some(0),
            false => {
                let mut left: usize = 0;
                let mut z: usize = 0;
                for k in 1..(self.pat.len() - 1) {
                    match k > right {
                        true => {
                            z += text[k..]
                                .chars()
                                .zip(self.pat.chars())
                                .take_while(|(a, b)| a == b)
                                .count();
                            match z == self.pat.len() {
                                true =>  return Some(k), 
                                _ => {
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
                                        true => return Some(k),
                                        _ => {}
                                    }
                                }
                                false => {
                                    z = beta
                                        + text[(right + 1)..]
                                            .chars()
                                            .zip(self.pat[z1..].chars())
                                            .take_while(|(a, b)| a == b)
                                            .count();
                                    left = k;
                                    right = left + z;
                                    match z == self.pat.len() {
                                        true => return Some(k),
                                        _ => {}
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
    #[test]
    fn linear_search_test_1() {
        let s = super::Searcher::from_str("abxyabxz");
        assert_eq!(s.search("xabxyabxyabxz"), Some(5));
        assert_eq!(s.search("xabxyabxyabx"), None);
        assert_eq!(s.search("xabxyabxyabxs"), None);
        assert_eq!(s.search("xabxybxyabxy"), None);
        assert_eq!(s.search("xabxycbxyabxy"), None);
    }
}
