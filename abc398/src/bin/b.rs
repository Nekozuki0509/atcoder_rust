use std::collections::BTreeMap;

use proconio::input;

fn main() {
    input! {
        a: [usize;7],
    }

    let mut b_tree = BTreeMap::new();

    for i in a {
        *b_tree.entry(i).or_insert(0) += 1;
    }

    let mut used = 0;
    for i in b_tree.clone() {
        if i.1 >= 3 {
            used = i.0;
        }
    }

    if used == 0 {
        println!("No");
        return;
    }

    for i in b_tree {
        if i.1 >= 2 && used != i.0 {
            println!("Yes");
            return;
        }
    }

    println!("No");
    
}
