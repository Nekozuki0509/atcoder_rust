use std::collections::BTreeSet;

#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        s: [String;3],
    }

    let mut set = BTreeSet::new();
    set.insert("ABC".to_string());
    set.insert("ARC".to_string());
    set.insert("AGC".to_string());
    set.insert("AHC".to_string());

    for i in s {
        set.remove(&i);
    }

    println!("{}", set.first().unwrap());
}
