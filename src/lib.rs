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
//! let result = fp_growth_str.find_frequent_patterns();
//! println!("The number of results: {}", result.frequent_patterns_num());
//! for (frequent_pattern, support) in result.frequent_patterns().iter() {
//!     println!("{:?} {}", frequent_pattern, support);
//! }
//! ```

use std::{fmt::Debug, hash::Hash};

pub mod algorithm;
pub mod tree;

pub trait ItemType: Eq + Ord + Hash + Copy + Debug {}

impl<T> ItemType for T where T: Eq + Ord + Hash + Copy + Debug {}

#[cfg(test)]
mod tests {
    use crate::algorithm::FPGrowth;
    use crate::tree::{Node, Tree};
    use std::rc::Rc;

    #[test]
    fn test_node() {
        let root_node = Node::<i32>::new_rc(None, 0);
        let child_node_1 = Rc::new(Node::<i32>::new(Some(1), 1));
        let child_node_2 = Rc::new(Node::<i32>::new(Some(2), 2));

        root_node.add_child(Rc::clone(&child_node_1));
        child_node_1.add_child(Rc::clone(&child_node_2));

        assert!(root_node.is_root());
        assert_eq!(root_node.search(1), Some(Rc::clone(&child_node_1)));
        assert_eq!(root_node.search(2), None);
        assert_eq!(root_node.item, None);

        assert!(!child_node_1.is_root());
        assert_eq!(child_node_1.search(1), None);
        assert_eq!(child_node_1.search(2), Some(Rc::clone(&child_node_2)));
        assert_eq!(child_node_1.item, Some(1));

        assert!(!child_node_2.is_root());
        assert_eq!(child_node_2.search(1), None);
        assert_eq!(child_node_2.search(2), None);
        assert_eq!(child_node_2.item, Some(2));
    }

    #[test]
    fn test_tree() {
        let mut tree = Tree::<&str>::new();
        let transactions = vec![
            vec!["a", "c", "e", "b", "f"],
            vec!["a", "c", "g"],
            vec!["e"],
            vec!["a", "c", "e", "g", "d"],
            vec!["a", "c", "e", "g"],
            vec!["e"],
            vec!["a", "c", "e", "b", "f"],
            vec!["a", "c", "d"],
            vec!["a", "c", "e", "g"],
            vec!["a", "c", "e", "g"],
        ];
        for transaction in transactions.into_iter() {
            tree.add_transaction(transaction);
        }
    }
    #[test]
    fn test_algorithm() {
        let transactions = vec![
            vec!["a", "c", "e", "b", "f", "h", "a", "e", "f"],
            vec!["a", "c", "g"],
            vec!["e"],
            vec!["e", "c", "a", "g", "d"],
            vec!["a", "c", "e", "g"],
            vec!["e", "e"],
            vec!["a", "c", "e", "b", "f"],
            vec!["a", "c", "d"],
            vec!["g", "c", "e", "a"],
            vec!["a", "c", "e", "g"],
            vec!["i"],
        ];
        // FIXME: use specific result cases to verify correctness.
        let test_cases: Vec<(usize, usize, usize)> = vec![
            // (minimum_support, frequent_patterns_num, elimination_set_num)
            (1, 88, 0),
            (2, 43, 2),
            (3, 15, 5),
            (4, 15, 5),
            (5, 11, 5),
            (6, 7, 9),
            (7, 4, 9),
            (8, 4, 9),
            (9, 0, 11),
        ];
        for (minimum_support, frequent_patterns_num, elimination_set_num) in test_cases.iter() {
            let fp_growth_str = FPGrowth::<&str>::new(transactions.clone(), *minimum_support);
            let result = fp_growth_str.find_frequent_patterns();
            assert_eq!(*frequent_patterns_num, result.frequent_patterns_num());
            assert_eq!(*elimination_set_num, result.elimination_set_num());
        }
    }
}
