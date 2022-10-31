//! module for string-searching algorithms

/// Module for preprocessing routines for linear and sublinear
/// search algorithms like Boyer Moore
mod preprocess;

/// Linear search based on Boyer Moore algorithm, additionally using preprocess-Z
mod boyer_moore;
pub use boyer_moore::BoyerMooreSearcher;

/// Simple linear-time exact string search using preprocess-Z
mod linear_search;
pub use linear_search::Searcher;

/// Naive search based on backtracking on first  mismatch
mod naive_search;
pub use naive_search::NaiveSearcher;
