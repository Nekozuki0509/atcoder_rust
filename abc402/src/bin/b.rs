use std::collections::{BTreeMap, VecDeque};

#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        q: usize,
    }

    let mut p = VecDeque::new();
    for _ in 1..=q {
        input! {a: usize}

        if a == 1 {
            input! {b: usize}
            p.push_back(b);
        } else {
            println!("{}", p.pop_front().unwrap());
        }
    }
}
