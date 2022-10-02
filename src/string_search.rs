//! module for string-searching algorithms
// search based on backtracking on first  mismatch
mod naive_search;
// preprocess-Z as common preprocessing for smarter algorithms
// e.g. BM, KMP, etc.
mod preprocess_z;

// Simple linear-time exact string search using preprocess-Z
mod linear_search;
