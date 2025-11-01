use std::collections::BTreeSet;

#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        (n, k): (usize, usize),
        mut a: [usize;n]
    }

    let sum = k * (k+1) / 2;
    let mut set = BTreeSet::new();

    for i in a {
        if i <= k {
            set.insert(i);
        }
    }
    println!("{}", sum-set.iter().sum::<usize>());
}
