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

    let mut an = 0;
    for i in 1usize..50000 {
        if let (Some(x), Some(y)) = (b.get(&i), b.get(&(100000 - i))) {
            an += x * y;
        }
    }

    let x = b.get(&50000).unwrap_or(&0);
    if x > &1 {
        an += x * (x - 1) / 2;
    }
    
    println!("{}", an);
}
