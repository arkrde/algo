//! Module containing naive string search with backtracking.
//! Time complexity = O(mn) where m and n are the lengths of patten
//! and text respectively.
pub struct NaiveSearcher<'a, T> {
    pat: &'a [T],
    text: &'a [T],
    cursor: usize,
}
impl<'a, T> NaiveSearcher<'a, T>
where
    T: Ord,
{
    /// Constructs a searcher from a generic sequence
    pub fn new(pat: &'a [T], text: &'a [T]) -> NaiveSearcher<'a, T> {
        NaiveSearcher {
            pat,
            text,
            cursor: 0,
        }
    }
}

impl<'a, T> Iterator for NaiveSearcher<'a, T>
where
    T: Ord,
{
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        if self.cursor + self.pat.len() >= self.text.len() {
            None
        } else {
            match Self::search_helper(&self.pat, &self.text[self.cursor..]) {
                Some(n) => {
                    self.cursor += n + 1;
                    Some(self.cursor - 1)
                }
                None => None,
            }
        }
    }
}

impl<'a, T> NaiveSearcher<'a, T>
where
    T: Ord,
{
    /// Method that implements search operation on some text.
    /// Returns an optional result containing the index to the
    /// beginning of a successful result.
    fn search_helper(pat: &'a [T], text: &'a [T]) -> Option<usize> {
        // let mut pos: usize = 0;
        for pos in 0..text.len() {
            //let c = pat
            //.chars()
            //.zip(text[pos..].chars())
            //.take_while(|(a, b)| a == b)
            //.count();
            let c = pat
                .iter()
                .zip(text[pos..].iter())
                .take_while(|(a, b)| a == b)
                .count();

            if c == pat.len() {
                return Some(pos);
            }
        }
        None
    }
}

#[cfg(test)]
mod string_algorithm_tests {
    use super::NaiveSearcher;
    #[test]
    fn naive_search_test_string() {
        let pat = "abxyabxz";
        let text = "xabxyabxyabxz";
        let mut s = NaiveSearcher::new(pat.as_bytes(), text.as_bytes());
        assert_eq!(s.next(), Some(5));
        let results: Vec<usize> =
            NaiveSearcher::new("ab".as_bytes(), "xabxyabxyabxz".as_bytes()).collect();
        assert_eq!(results, vec![1, 5, 9]);
        assert_eq!(
            NaiveSearcher::new("abxyabxz".as_bytes(), "xabxyabxyabxz".as_bytes())
                .collect::<Vec<usize>>(),
            vec![5]
        );
        assert_eq!(
            NaiveSearcher::new("abxyabxz".as_bytes(), "xabxyabxyabx".as_bytes())
                .collect::<Vec<usize>>(),
            vec![]
        );
        assert_eq!(
            NaiveSearcher::new("abxyabxz".as_bytes(), "xabxyabxyabxs".as_bytes())
                .collect::<Vec<usize>>(),
            vec![]
        );
        assert_eq!(
            NaiveSearcher::new("abxyabxz".as_bytes(), "xabxybxyabxy".as_bytes())
                .collect::<Vec<usize>>(),
            vec![]
        );
        assert_eq!(
            NaiveSearcher::new("abxyabxz".as_bytes(), "xabxycbxyabxy".as_bytes())
                .collect::<Vec<usize>>(),
            vec![]
        );
    }
    #[test]
    fn naive_search_test_integer() {
        assert_eq!(
            NaiveSearcher::new(&vec![1, 2, 3, 4, 5, 6], &vec![7, 8, 9]).collect::<Vec<usize>>(),
            vec![]
        );
        assert_eq!(
            NaiveSearcher::new(&vec![1, 2, 3, 4, 5, 6], &vec![7, 8, 9, 10, 11, 12, 13])
                .collect::<Vec<usize>>(),
            vec![]
        );
        assert_eq!(
            NaiveSearcher::new(&vec![1, 2], &vec![1, 2, 3, 4, 1, 2]).collect::<Vec<usize>>(),
            vec![0, 4]
        );
    }
}
