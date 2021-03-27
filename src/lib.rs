//! A lib that implements the FP-Growth algorithm.
//!
//! # Usage
//!
//! To get you started quickly, the easiest and highest-level way to get
//! a FP-Growth algorithm result is to use [`algorithm::FPGrowth::find_frequent_patterns()`]
//!
//! ```
//! use fp_growth::algorithm::*;
//!
//! let transactions = vec![
//!     vec!["e", "c", "a", "b", "f", "h"],
//!     vec!["a", "c", "g"],
//!     vec!["e"],
//!     vec!["e", "c", "a", "g", "d"],
//!     vec!["a", "c", "e", "g"],
//!     vec!["e"],
//!     vec!["a", "c", "e", "b", "f"],
//!     vec!["a", "c", "d"],
//!     vec!["g", "c", "e", "a"],
//!     vec!["a", "c", "e", "g"],
//!     vec!["i"],
//! ];
//! let minimum_support = 2;
//! let fp_growth_str = FPGrowth::<&str>::new(transactions, minimum_support);
//! let results = fp_growth_str.find_frequent_patterns();
//! println!("The number of results: {}", &results.len());
//! for (frequent_pattern, support) in results.iter() {
//!     println!("{:?} {}", frequent_pattern, support);
//! }
//! ```

pub mod algorithm;
pub mod tree;
