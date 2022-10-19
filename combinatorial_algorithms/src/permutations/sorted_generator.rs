//! Iterator that produces all permutations of a sequence
//! in a lexicographically sorted order.

/// This implements a generator that generates all permutations of a sequence 
/// in a lexicographic sorted order using "Algorithm-L" by Knuth
struct Generator<T> {
    seq: Vec<T>,
    started: bool,
}

impl<T> Generator<T>
where
    T: Ord + Clone,
{
    pub fn new<Iter: Iterator<Item = T>>(seq: Iter) -> Generator<T> {
        let mut seq: Vec<T> = seq.into_iter().collect();
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
                .take_while(|(a, b)| a < b)
                .count();
            if shift == n - 1 {
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
        let mut generator = Generator::new("abcd".chars());
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
}
