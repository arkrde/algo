//! module for string-searching algorithms

/// Module for preprocessing routines for linear and sublinear
/// search algorithms like Boyer Moore
mod preprocess;

/// Naive search based on backtracking on first  mismatch
pub mod naive_search;
/// Simple linear-time exact string search using preprocess-Z
pub mod linear_search;
/// Linear search based on Boyer Moore algorithm, additionally using preprocess-Z
pub mod boyer_moore;