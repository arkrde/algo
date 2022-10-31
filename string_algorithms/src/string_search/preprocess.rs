// preprocess-Z as common preprocessing for smarter algorithms
// e.g. BM, KMP, etc.
mod preprocess_z;
pub use preprocess_z::PrepZ;

// preprocess-N for Boyer Moore
mod preprocess_n;
pub use preprocess_n::PrepN;

// preprocess-L for Boyer Moore
mod preprocess_l;
pub use preprocess_l::PrepL;

// preprocess-Lprime for Boyer Moore
mod preprocess_lprime;
pub use preprocess_lprime::PrepLp;

// preprocess-R for Boyer Moore
mod preprocess_r;
pub use preprocess_r::PrepR;
