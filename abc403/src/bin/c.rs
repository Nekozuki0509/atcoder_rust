use std::collections::BTreeSet;

use amplify::confinement::Collection;
#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        q: usize
    }

    let mut vec = vec![BTreeSet::new();n+1];

    for _ in 1..=q {
        input! {i: usize}
        match i {
            1 => {
                input! {x: usize, y: isize}
                vec[x].push(y);
            },
            2 => {
                input! {x: usize}
                vec[x].push(-1);
            },
            3 => {
                input! {x: usize, y: isize}
                let an = if vec[x].contains(&-1) || vec[x].contains(&y) {
                    "Yes"
                } else {
                    "No"
                };
                println!("{}", an);
            },
            _ => {}
        }
    }
}
