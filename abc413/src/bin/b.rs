use std::collections::BTreeSet;

#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: [String;n]
    }

    let mut set = BTreeSet::new();
    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }

            set.insert(s[i].clone()+&s[j]);
        }
    }

    println!("{}", set.len());
}
