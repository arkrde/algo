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
        for _ in 0..1000 {
            let result: Vec<usize> =
                naive_search::NaiveSearcher::new(pat.as_bytes(), text.as_bytes()).collect();
            counter += result.len();
        }
    }
    println!("{}", counter);
    // {
    //     for _ in 0..1000 {
    //         let _: Vec<usize> = boyer_moore::BoyerMooreSearcher::new(pat, text).collect();
    //     }
    // }
}
