use std::collections::BTreeSet;

use num::abs;
#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        (x, y): (isize, isize)
    }

    let mut an = BTreeSet::new();

    for i in 1..=6 {
        for j in 1..=6 {
            if i + j >= x || abs(i - j) >= y {
                an.insert((i, j));
            }
        }
    }

    println!("{}", an.len() as f64 / 36.0);
}
