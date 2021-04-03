use fp_growth::algorithm::FPGrowth;

fn main() {
    let transactions = vec![
        vec!["e", "c", "a", "b", "f", "h"],
        vec!["a", "c", "g"],
        vec!["e"],
        vec!["e", "c", "a", "g", "d"],
        vec!["a", "c", "e", "g"],
        vec!["e"],
        vec!["a", "c", "e", "b", "f"],
        vec!["a", "c", "d"],
        vec!["g", "c", "e", "a"],
        vec!["a", "c", "e", "g"],
        vec!["i"],
    ];
    let minimum_support = 2;
    let fp_growth_str = FPGrowth::<&str>::new(transactions, minimum_support);

    let result = fp_growth_str.find_frequent_patterns();
    println!("The number of results: {}", result.frequent_patterns_num());
    for (frequent_pattern, support) in result.frequent_patterns().iter() {
        println!("{:?} {}", frequent_pattern, support);
    }
}
