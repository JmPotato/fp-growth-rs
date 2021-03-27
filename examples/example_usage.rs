use fp_growth::algorithm::FPGrowth;

fn main() {
    let transactions = vec![
        vec!["c", "e", "a", "b", "f"],
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
    let minimum_support = 2;
    let fp_growth_str = FPGrowth::<&str>::new(transactions, minimum_support);

    let results = fp_growth_str.find_frequent_patterns();
    println!("The number of results: {}", &results.len());
    for (frequent_pattern, support) in results.iter() {
        println!("{:?} {}", frequent_pattern, support);
    }
}
