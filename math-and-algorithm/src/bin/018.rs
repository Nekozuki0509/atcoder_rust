use std::collections::BTreeMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize;n]
    }

    let mut b: BTreeMap<usize, usize> = BTreeMap::new();
    for i in a {
        *b.entry(i).or_insert(0) += 1;
    }
    
    println!("{}", b.get(&100).unwrap_or(&0) * b.get(&400).unwrap_or(&0) + b.get(&200).unwrap_or(&0) * b.get(&300).unwrap_or(&0));
}
