//! `algorithm` is the core module of FP-Growth algorithm.
//! It implements the algorithm based on the internal data structs [`crate::tree::Node<T>`] and [`crate::tree::Tree<T>`].

use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    usize,
};

use crate::tree::Tree;
use crate::ItemType;

#[allow(clippy::upper_case_acronyms)]
#[derive(Clone, Debug)]
pub struct FPResult<T> {
    frequent_patterns: Vec<(Vec<T>, usize)>,
    elimination_sets: HashSet<Vec<T>>,
}

impl<T: ItemType> FPResult<T> {
    pub fn new(
        frequent_patterns: Vec<(Vec<T>, usize)>,
        elimination_sets: HashSet<Vec<T>>,
    ) -> FPResult<T> {
        FPResult {
            frequent_patterns,
            elimination_sets,
        }
    }

    pub fn frequent_patterns_num(&self) -> usize {
        self.frequent_patterns.len()
    }

    pub fn frequent_patterns(&self) -> Vec<(Vec<T>, usize)> {
        self.frequent_patterns.clone()
    }

    pub fn elimination_sets_num(&self) -> usize {
        self.elimination_sets.len()
    }

    pub fn elimination_sets(&self) -> Vec<Vec<T>> {
        self.elimination_sets.clone().into_iter().collect()
    }
}

/// `FPGrowth<T>` represents an algorithm instance, it should include the `transactions` input
/// and minimum support value as the initial config. Once it is created, you could run
/// [`FPGrowth::find_frequent_patterns()`] to start the frequent pattern mining.
// `transactions` will be sorted and deduplicated before starting the algorithm.
#[allow(clippy::upper_case_acronyms)]
pub struct FPGrowth<T> {
    transactions: Vec<Vec<T>>,
    minimum_support: usize,
}

impl<T: ItemType> FPGrowth<T> {
    /// Create a FP-Growth algorithm instance with the given `transactions` and `minimum_support`.
    pub fn new(transactions: Vec<Vec<T>>, minimum_support: usize) -> FPGrowth<T> {
        FPGrowth {
            transactions,
            minimum_support,
        }
    }

    /// Find frequent patterns in the given transactions using FP-Growth.
    pub fn find_frequent_patterns(&self) -> FPResult<T> {
        // Collect and preprocess the transactions.
        let mut items = HashMap::new();
        for transaction in self.transactions.clone().into_iter() {
            let mut item_set: HashSet<T> = HashSet::new();
            for &item in transaction.iter() {
                // Check whether we have inserted the same item in a transaction before,
                // make sure we won't calculate the wrong support.
                match item_set.contains(&item) {
                    true => continue,
                    false => {
                        item_set.insert(item);
                        let count = items.entry(item).or_insert(0);
                        *count += 1;
                    }
                };
            }
        }

        // Clean up the items whose support is lower than the minimum_support.
        let cleaned_items: HashMap<&T, &usize> = items
            .iter()
            .filter(|(_, &count)| count >= self.minimum_support)
            .collect();
        let mut elimination_sets = HashSet::new();

        let mut tree = Tree::<T>::new();
        for transaction in self.transactions.clone().into_iter() {
            let mut cleaned_transaction: Vec<T> = transaction
                .clone()
                .into_iter()
                .filter(|item| cleaned_items.contains_key(item))
                .collect();
            if cleaned_transaction.len() != transaction.len() {
                elimination_sets.insert(transaction);
            }
            cleaned_transaction.sort_by(|a, b| {
                let &a_counter = cleaned_items.get(a).unwrap();
                let &b_counter = cleaned_items.get(b).unwrap();
                match b_counter.cmp(a_counter) {
                    Ordering::Equal => {
                        // When counter is the same, we will sort by T itself.
                        // e.g. ["c", "b", "a"] -> ["a", "b", "c"]
                        match b.cmp(a) {
                            Ordering::Greater => Ordering::Less,
                            Ordering::Less => Ordering::Greater,
                            Ordering::Equal => Ordering::Equal,
                        }
                    }
                    Ordering::Less => Ordering::Less,
                    Ordering::Greater => Ordering::Greater,
                }
            });
            // After sort cleaned_transaction, remove consecutive items from it then.
            cleaned_transaction.dedup();
            tree.add_transaction(cleaned_transaction);
        }

        let mut fp_result = self.find_with_suffix(&tree, &[]);
        fp_result.elimination_sets.extend(elimination_sets);
        fp_result
    }

    fn find_with_suffix(&self, tree: &Tree<T>, suffix: &[T]) -> FPResult<T> {
        let mut fp_result = FPResult::new(vec![], HashSet::new());
        for (item, nodes) in tree.get_all_items_nodes().iter() {
            let mut support = 0;
            for node in nodes.iter() {
                support += node.count();
            }
            let mut frequent_pattern = vec![*item];
            frequent_pattern.append(&mut Vec::from(suffix));
            if support >= self.minimum_support && !suffix.contains(item) {
                fp_result
                    .frequent_patterns
                    .push((frequent_pattern.clone(), support));

                let partial_tree = Tree::generate_partial_tree(&tree.generate_prefix_path(*item));
                let mut mid_fp_result = self.find_with_suffix(&partial_tree, &frequent_pattern);
                fp_result
                    .frequent_patterns
                    .append(&mut mid_fp_result.frequent_patterns);
                fp_result
                    .elimination_sets
                    .extend(mid_fp_result.elimination_sets);
            } else {
                fp_result.elimination_sets.insert(frequent_pattern);
            }
        }
        fp_result
    }
}
