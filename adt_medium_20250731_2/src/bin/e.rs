use std::collections::BTreeSet;

#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        (n, m, q): (usize, usize, usize)
    }

    let mut vec = vec![BTreeSet::new();n+1];
    for _ in 0..q {
        input! {t: u8, x: usize}
        match t {
            1 => {
                input! {y: isize}
                vec[x].insert(y);
            },
            2 => {
                vec[x].insert(-1);
            },
            _ => {
                input! {y: isize}
                if vec[x].contains(&-1) || vec[x].contains(&y) {
                    println!("Yes");
                } else {
                    println!("No");
                }
            }
        }
    }
}
