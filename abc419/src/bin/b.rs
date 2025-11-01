use std::collections::BTreeSet;

#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        q: usize,
    }

    let mut balls = vec![];
    for _ in 0..q {
        input! {t: u8}
        match t {
            1 => {
                input! {x: usize}
                balls.push(x);
                balls.sort();
            },
            _ => {
                println!("{}", balls.remove(0));
            }
        }
    }
}
