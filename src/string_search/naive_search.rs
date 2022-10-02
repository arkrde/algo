//! Module containing naive string search with backtracking.
//! Time complexity = O(mn) where m and n are the lengths of patten
//! and text respectively.
pub struct Searcher {
    pattern: String,
}
impl Searcher {
    /// Constructs a searcher from a string
    pub fn from_str(pat: &str) -> Searcher {
        Searcher {
            pattern: pat.to_string(),
        }
    }
    /// Method that implements search operation on some text.
    /// Returns an optional result containing the index to the
    /// beginning of a successful result.
    pub fn search(&self, text: &str) -> Option<usize> {
        let mut pos: usize = 0;
        while pos <= text.len() - self.pattern.len() {
            let mut done: bool = true;
            for (p, t) in self.pattern.chars().zip(text[pos..].chars()) {
                if p != t {
                    done = false;
                    break;
                }
            }
            if done {
                return Some(pos);
            }
            pos += 1;
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::string_search::naive_search;
    #[test]
    fn search_test() {
        let s = naive_search::Searcher::from_str("abxyabxz");
        let res = s.search("xabxyabxyabxz");
        assert_eq!(res, Some(5));
        let s = naive_search::Searcher::from_str("abxybxz");
        let res = s.search("xabxyabxyabxz");
        assert_eq!(res, None);
    }
}
