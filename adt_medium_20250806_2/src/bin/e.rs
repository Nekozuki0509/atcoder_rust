use std::collections::BTreeSet;

use amplify::confinement::Collection;
#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        (n, m, k): (usize, usize, usize),
    }

    let mut corrects = BTreeSet::new();
    let mut incorrects = BTreeSet::new();
    let mut precorrects = vec![];
    for _ in 0..m {
        input! {
            c: usize,
            a: [usize;c],
            r: char
        }        

        if k <= a.iter().filter(|&x| corrects.contains(x)).count() && r.eq(&'x') {
            println!("0");
            return;
        }

        if c < k {
            continue;
        }

        if c == k && r.eq(&'o') {
            for i in a {
                corrects.push(i);
            }

            continue;
        }
    }
}
