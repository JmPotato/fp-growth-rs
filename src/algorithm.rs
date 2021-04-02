//! `algorithm` is the core module of FP-Growth algorithm.
//! It implements the algorithm based on the internal data structs [`crate::tree::Node<T>`] and [`crate::tree::Tree<T>`].

use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    usize,
};

use crate::tree::Tree;
use crate::ItemType;

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
    pub fn find_frequent_patterns(&self) -> Vec<(Vec<T>, usize)> {
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

        let mut tree = Tree::<T>::new();
        for transaction in self.transactions.clone().into_iter() {
            let mut cleaned_transaction: Vec<T> = transaction
                .into_iter()
                .filter(|item| cleaned_items.contains_key(item))
                .collect();
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

        self.find_with_suffix(&tree, &[])
    }

    fn find_with_suffix(&self, tree: &Tree<T>, suffix: &[T]) -> Vec<(Vec<T>, usize)> {
        let mut results = vec![];
        for (item, nodes) in tree.get_all_items_nodes().iter() {
            let mut support = 0;
            for node in nodes.iter() {
                support += node.count();
            }
            if support >= self.minimum_support && !suffix.contains(item) {
                let mut frequent_pattern = vec![*item];
                frequent_pattern.append(&mut Vec::from(suffix));
                results.push((frequent_pattern.clone(), support));

                let partial_tree = Tree::generate_partial_tree(&tree.generate_prefix_path(*item));
                results.append(&mut self.find_with_suffix(&partial_tree, &frequent_pattern));
            }
        }
        results
    }
}
