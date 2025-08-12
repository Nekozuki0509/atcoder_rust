use std::collections::{BTreeMap, BTreeSet};

use num::abs;
#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        n: usize
    }

    let mut an = 0;
    let mut map = BTreeMap::new();
    for i in 0..n as isize {
        input! {a: isize}
        an += map.get(&(i - a)).unwrap_or(&0);
        *map.entry(i + a).or_insert(0usize) += 1;
    }

    println!("{}", an);
}
