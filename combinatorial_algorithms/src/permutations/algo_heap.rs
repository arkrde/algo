//! Implementation of a permutation generator using Heap's algorithm
struct Generator<T> {
    seq: Vec<T>,
}

impl<T> Generator<T>
where
    T: PartialOrd,
{
    pub fn new<Iter : Iterator<Item = T>>(seq: Iter) -> Generator<T> {
        let seq: Vec<T> = seq.collect();
        Generator { seq }
    }
}

// impl<T> Iterator for Permutation<T>
// where T: PartialOrd{
//     type Item = Vec<T>;
//     fn next(&mut self) -> Option<Self::Item> {
//         Some(self.seq)
//     }
// }

#[cfg(test)]
mod permutation_test {
    use super::*;
    #[test]
    fn permutation_test() {
        let generator = Generator::new("abcd".chars());
        assert_eq!(generator.seq, "abcd".chars().collect::<Vec<char>>());
    }
}
