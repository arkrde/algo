//! Module containing naive string search with backtracking.
//! Time complexity = O(mn) where m and n are the lengths of patten
//! and text respectively.
pub struct NaiveSearcher<'a> {
    pat: &'a str,
    text: &'a str,
    pos: usize,
}
impl<'a> NaiveSearcher<'a> {
    /// Constructs a searcher from a string
    pub fn new(pat: &'a str, text: &'a str) -> NaiveSearcher<'a> {
        let pos = 0;
        NaiveSearcher { pat, text, pos }
    }
}

impl<'a> Iterator for NaiveSearcher<'a> {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        if self.pos >= (self.text.len() - self.pat.len()) {
            None
        } else {
            match Self::search_helper(&self.pat, &self.text[self.pos..]) {
                Some(n) => {
                    self.pos += n + 1;
                    Some(self.pos - 1)
                }
                None => None,
            }
        }
    }
}

impl<'a> NaiveSearcher<'a> {
    /// Method that implements search operation on some text.
    /// Returns an optional result containing the index to the
    /// beginning of a successful result.
    fn search_helper(pat: &str, text: &str) -> Option<usize> {
        // let mut pos: usize = 0;
        for pos in 0..(text.len() - pat.len() + 1) {
            let c = pat
                .chars()
                .zip(text[pos..].chars())
                .take_while(|(a, b)| a == b)
                .count();

            match c == pat.len() {
                true => return Some(pos),
                _ => {}
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn search_test() {
        let pat = "abxyabxz";
        let text = "xabxyabxyabxz";
        let mut s = super::NaiveSearcher::new(pat, text);
        assert_eq!(s.next(), Some(5));
        let results: Vec<usize> = super::NaiveSearcher::new("ab", "xabxyabxyabxz").collect();
        assert_eq!(results, vec![1, 5, 9]);
        assert_eq!(super::NaiveSearcher::new("abxyabxz", "xabxyabxyabxz").collect::<Vec<usize>>(), vec![5]);
        assert_eq!(super::NaiveSearcher::new("abxyabxz", "xabxyabxyabx").collect::<Vec<usize>>(), vec![]);
        assert_eq!(super::NaiveSearcher::new("abxyabxz", "xabxyabxyabxs").collect::<Vec<usize>>(), vec![]);
        assert_eq!(super::NaiveSearcher::new("abxyabxz", "xabxybxyabxy").collect::<Vec<usize>>(), vec![]);
        assert_eq!(super::NaiveSearcher::new("abxyabxz", "xabxycbxyabxy").collect::<Vec<usize>>(), vec![]);
    }
}
