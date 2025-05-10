use std::collections::BTreeMap;

use itertools::Itertools;
#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize;n]
    }

    let mut map = BTreeMap::new();
    for i in 1..=m {
        map.insert(i, 0);
    }

    for i in &a {
        *map.entry(*i).or_insert(0) += 1;
    }

    for (i, v) in a.iter().rev().enumerate() {
        if map.values().contains(&0) {
            println!("{}", i);
            return;
        } else {
            *map.entry(*v).or_insert(1) -= 1;
        }
    }

    println!("{}", a.len());
}
