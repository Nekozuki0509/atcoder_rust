use std::collections::BTreeMap;

#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        s: Chars,
    }

    let mut vec = BTreeMap::new();
    for i in s {
        *vec.entry(i).or_insert(0) += 1;
    }

    for i in vec {
        if i.1 == 1 {
            println!("{}", i.0);
            return;
        }
    }
}
