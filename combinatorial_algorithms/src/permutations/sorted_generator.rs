//! Iterator that produces all permutations of a sequence
//! in a lexicographically sorted order.

/// This implements a generator that generates all permutations of a sequence
/// in a lexicographic sorted order using "Algorithm-L" by Knuth
struct Generator<T> {
    seq: Vec<T>,
    started: bool,
}

impl<T> FromIterator<T> for Generator<T>
where
    T: Ord,
{
    fn from_iter<Iter: IntoIterator<Item = T>>(iter: Iter) -> Self {
        let mut seq: Vec<T> = Vec::from_iter(iter);
        seq.sort();
        Generator {
            seq,
            started: false,
        }
    }
}

impl<T> Iterator for Generator<T>
where
    T: Ord + Clone,
{
    type Item = Vec<T>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.started == false {
            self.started = true;
            Some(self.seq.clone())
        } else {
            let n = self.seq.len();
            let shift = self
                .seq
                .iter()
                .rev()
                .zip(self.seq[..(n - 1)].iter().rev())
                .take_while(|(a, b)| a <= b)
                .count();
            if shift >= n - 1 {
                None
            } else {
                let j = n - 2 - shift;
                let shift = self
                    .seq
                    .iter()
                    .rev()
                    .take_while(|v| v <= &&self.seq[j])
                    .count();
                let l = n - 1 - shift;
                self.seq.swap(j, l);
                let mut k = j + 1;
                let mut l = n - 1;
                while k < l {
                    self.seq.swap(k, l);
                    k += 1;
                    l -= 1;
                }
                Some(self.seq.clone())
            }
        }
    }
}

#[cfg(test)]
mod permutation_test {
    use super::*;
    #[test]
    fn permutation_test_unique() {
        let mut generator = Generator::from_iter("abcd".chars());
        assert_eq!(generator.next().unwrap().iter().collect::<String>(), "abcd");
        assert_eq!(generator.next().unwrap().iter().collect::<String>(), "abdc");
        assert_eq!(generator.next().unwrap().iter().collect::<String>(), "acbd");
        assert_eq!(generator.next().unwrap().iter().collect::<String>(), "acdb");
        assert_eq!(generator.next().unwrap().iter().collect::<String>(), "adbc");
        assert_eq!(generator.next().unwrap().iter().collect::<String>(), "adcb");
        assert_eq!(generator.next().unwrap().iter().collect::<String>(), "bacd");
        assert_eq!(generator.next().unwrap().iter().collect::<String>(), "badc");
        assert_eq!(generator.next().unwrap().iter().collect::<String>(), "bcad");
        assert_eq!(generator.next().unwrap().iter().collect::<String>(), "bcda");
        assert_eq!(generator.next().unwrap().iter().collect::<String>(), "bdac");
        assert_eq!(generator.next().unwrap().iter().collect::<String>(), "bdca");
        assert_eq!(generator.next().unwrap().iter().collect::<String>(), "cabd");
        assert_eq!(generator.next().unwrap().iter().collect::<String>(), "cadb");
        assert_eq!(generator.next().unwrap().iter().collect::<String>(), "cbad");
        assert_eq!(generator.next().unwrap().iter().collect::<String>(), "cbda");
        assert_eq!(generator.next().unwrap().iter().collect::<String>(), "cdab");
        assert_eq!(generator.next().unwrap().iter().collect::<String>(), "cdba");
        assert_eq!(generator.next().unwrap().iter().collect::<String>(), "dabc");
        assert_eq!(generator.next().unwrap().iter().collect::<String>(), "dacb");
        assert_eq!(generator.next().unwrap().iter().collect::<String>(), "dbac");
        assert_eq!(generator.next().unwrap().iter().collect::<String>(), "dbca");
        assert_eq!(generator.next().unwrap().iter().collect::<String>(), "dcab");
        assert_eq!(generator.next().unwrap().iter().collect::<String>(), "dcba");
        assert_eq!(generator.next(), None);
    }
    #[test]
    fn permutation_test_nonunique() {
        let seq: Vec<i32> = vec![1, 2, 2, 3];
        let mut generator: Generator<i32> = seq.into_iter().collect();
        assert_eq!(generator.next(), Some(vec![1, 2, 2, 3]));
        assert_eq!(generator.next(), Some(vec![1, 2, 3, 2]));
        assert_eq!(generator.next(), Some(vec![1, 3, 2, 2]));
        assert_eq!(generator.next(), Some(vec![2, 1, 2, 3]));
        assert_eq!(generator.next(), Some(vec![2, 1, 3, 2]));
        assert_eq!(generator.next(), Some(vec![2, 2, 1, 3]));
        assert_eq!(generator.next(), Some(vec![2, 2, 3, 1]));
        assert_eq!(generator.next(), Some(vec![2, 3, 1, 2]));
        assert_eq!(generator.next(), Some(vec![2, 3, 2, 1]));
        assert_eq!(generator.next(), Some(vec![3, 1, 2, 2]));
        assert_eq!(generator.next(), Some(vec![3, 2, 1, 2]));
        assert_eq!(generator.next(), Some(vec![3, 2, 2, 1]));
    }
    #[test]
    fn permutation_test_all_identical() {
        let seq: Vec<i32> = vec![1; 3];
        let generator: Generator<i32> = seq.into_iter().collect();
        let results: Vec<Vec<i32>> = generator.collect();
        assert_eq!(results.len(), 1);
    }
    #[test]
    fn permutation_test_short_sequence() {
        let generator: Generator<char> = Generator::from_iter("ab".chars());
        let results: Vec<Vec<char>> = generator.collect();
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn permutation_test_short_nonunique_sequence() {
        let generator: Generator<char> = Generator::from_iter("aa".chars());
        let results: Vec<Vec<char>> = generator.collect();
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn permutation_test_unitlength_sequence() {
        let generator: Generator<char> = "a".chars().collect();
        let results: Vec<Vec<char>> = generator.collect();
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn permutation_test_sort_test() {
        let mut generator : Generator<_> = "dcba".chars().collect();
        let first_result : String = generator.nth(0).unwrap().iter().collect();
        assert_eq!(first_result, "abcd");
    }
}
