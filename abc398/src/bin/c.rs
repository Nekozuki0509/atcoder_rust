use std::{collections::BTreeMap, ops::Index};

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [isize;n]
    }

    let mut b_tree = BTreeMap::new();

    for i in a.clone() {
        *b_tree.entry(i).or_insert(0) += 1;
    }

    let mut an = -1;

    for i in b_tree {
        if i.1 == 1 && i.0 > an {
            an = i.0;
        }
    }

    
    let ind = a.iter().position(|&x| x == an);

    let ann;
    if ind.is_none() {
        ann = -1;
    } else {
        ann = ind.unwrap() as i32 + 1;
    }
    
    println!("{}", ann);
}
