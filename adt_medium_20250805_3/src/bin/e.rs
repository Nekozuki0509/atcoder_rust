use std::collections::BTreeMap;

#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    let mut map = BTreeMap::new();
    for _ in 0..n {
        input! {s: String}
        if let Some(x) = map.get_mut(&s) {
            println!("{}({})", s, x);
            *x += 1;
        } else {
            println!("{}", s);
            map.insert(s, 1);
        }
    }
}
