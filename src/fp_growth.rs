use std::{collections::HashMap, fmt::Debug, hash::Hash, usize};

use crate::tree::Tree;

pub struct FPGrowth<T> {
    transactions: Vec<Vec<T>>,
    minimum_support: usize,
}

impl<T> Iterator for FPGrowth<T> {
    type Item = (Vec<T>, usize);

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

impl<T> FPGrowth<T>
where
    T: Eq + Hash + Debug + Copy,
{
    pub fn new(transactions: Vec<Vec<T>>, minimum_support: usize) -> FPGrowth<T> {
        FPGrowth {
            transactions,
            minimum_support,
        }
    }

    pub fn find_frequent_pattern(&self) /*-> Vec<(Vec<T>, usize)>*/
    {
        // Collect the transaction.
        let mut items = HashMap::new();
        for transaction in self.transactions.iter() {
            for &item in transaction.iter() {
                let count = items.entry(item).or_insert(0);
                *count += 1;
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
                .clone()
                .into_iter()
                .filter(|item| cleaned_items.contains_key(item))
                .collect();
            cleaned_transaction.sort_by(|a, b| {
                let &a_counter = cleaned_items.get(a).unwrap();
                let &b_counter = cleaned_items.get(b).unwrap();
                b_counter.cmp(a_counter)
            });
            tree.add_transaction(cleaned_transaction);
        }

        // Todo: implement the core algorithm.
    }
}

#[cfg(test)]
mod tests {
    use crate::fp_growth::FPGrowth;

    #[test]
    fn test_node() {
        let transactions = vec![vec!["b", "a", "c"], vec!["e", "a", "b"], vec!["f", "a"]];
        let minimum_support = 2;
        let fp_growth_str = FPGrowth::<&str>::new(transactions, minimum_support);

        fp_growth_str.find_frequent_pattern();
    }
}
