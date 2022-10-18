extern crate string_algorithms;
use string_algorithms::string_search::*;

fn main() {
    let text = "MDSKGSSQKGSRLLLLLVVSNLLLCQGVVSTPVCPNGPGNCQVSLRDLFDRAVMVSHYIHDLSS\
        EMFNEFDKRYAQGKGFITMALNSCHTSSLPTPEDKEQAQQTHHEVLMSLILGLLRSWNDPLYHL\
        VTEVRGMKGAPDAILSRAIEIEEENKRLLEGMEMIFGQVIPGAKETEPYPVWSGLPSLQTKDED\
        ARYSAFYNLLHCLRRDSSKIDTYLKLLNCRIIYNNNC";
    let pat = "LLLLLVVSNLLLCQGVVSTPVCPNGPGNCQVS";
    let mut counter = 0;
    {
        for _ in 0..1000000 {
            let res: Vec<usize> = boyer_moore::BoyerMooreSearcher::new(pat, text).collect();
            counter += res.len();
        }
    }
    println!("{}", counter);
}
