//! Given two strings: alpha and beta, this examples determines whether alpha
//! is a circular / cyclic rotation of beta.
extern crate algorithms;

use algorithms::string_search::linear_search;

/// To detect whether a string alpha is a circular rotation of another string beta, 
/// this algorithm first checks whether alpha and beta are of the same length. Then
/// it concatenates beta with itself, followed by an exact string search of alpha 
/// into the newly concatenated beta. If there is a match, then alpha is circular
/// rotation of beta. Otherwise, it's not.
/// 
/// The algorithm can use any exact-search algorithm. However, we use a simple 
/// preprocessing-Z based linear-search algorithm for efficiency.
fn main() {
    let alpha = "defabc";
    let beta = "abcdef";
    match alpha.len() == beta.len() {
        false => {
            println!("{} is not a circular rotation of {}", alpha, beta);
        },
        _ => {
            let full_string = beta.to_string() + beta;
            let searcher = linear_search::Searcher::from_str(&full_string[..]);
            match searcher.search(&full_string[..]) {
                Some(_) => {
                        println!("{} is a circular rotation of {}", alpha, beta);
                    },
                None => {
                    println!("{} is not a circular rotation of {}", alpha, beta);

                }
            }
        }
    }
}
