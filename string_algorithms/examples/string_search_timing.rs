extern crate string_algorithms;
use std::time::Instant;
use string_algorithms::string_search::*;

fn main() {
    let text = "MDSKGSSQKGSRLLLLLVVSNLLLCQGVVSTPVCPNGPGNCQVSLRDLFDRAVMVSHYIHDLSS\
        EMFNEFDKRYAQGKGFITMALNSCHTSSLPTPEDKEQAQQTHHEVLMSLILGLLRSWNDPLYHL\
        VTEVRGMKGAPDAILSRAIEIEEENKRLLEGMEMIFGQVIPGAKETEPYPVWSGLPSLQTKDED\
        ARYSAFYNLLHCLRRDSSKIDTYLKLLNCRIIYNNNC";
    let pat = "LLLLLVVSNLLLCQGVVSTPVCPNGPGNCQVS";
    {
        let now = Instant::now();
        for _ in 0..1000 {
            let _: Vec<usize> = naive_search::NaiveSearcher::new(pat, text).collect();
        }
        println!("Naive search: {:?}", now.elapsed() / 100);
    }
    {
        let now = Instant::now();
        for _ in 0..1000 {
            let _: Vec<usize> = boyer_moore::BoyerMooreSearcher::new(pat, text).collect();
        }
        println!("Boyer Moore: {:?}", now.elapsed() / 100);
    }
}
