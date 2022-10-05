//! module for string-searching algorithms
// search based on backtracking on first  mismatch
pub mod naive_search;
// preprocess-Z as common preprocessing for smarter algorithms
// e.g. BM, KMP, etc.
mod preprocess_z;

// preprocess-N as preprocessing for Boyer Moore
mod preprocess_n;

// preprocess-L as preprocessing for Boyer Moore
mod preprocess_l;

// preprocess-Lprime as preprocessing for Boyer Moore
mod preprocess_lprime;
// Simple linear-time exact string search using preprocess-Z
pub mod linear_search;

// preprocessor for Boyer-Moore search.
// mod bm_prep;