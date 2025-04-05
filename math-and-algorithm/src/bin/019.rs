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
    for i in b {
        if i.1 < 2 {
            continue;
        }
        an += i.1 * (i.1 - 1) / 2;
    }
    
    println!("{}", an);
}
