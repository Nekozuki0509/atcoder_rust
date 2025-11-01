#![allow(unused_imports, dead_code, non_snake_case)]
#![allow(unused_variables, unused_assignments)]
use amplify::confinement::Collection;
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1, Isize1}};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

#[fastout]
fn main() {
    input! {
        (n, m): (usize, usize),
        s: [Chars;n]
    }

    let mut set = BTreeSet::new();
    for i in 0..=n-m {
        for j in 0..=n-m {
            let mut v = vec![vec![];m];
            for k in 0..m {
                for l in 0..m {
                    v[k].push(s[i+k][j+l]);
                }
            }

            set.push(v);
        }
    }

    //dbg!(&set);

    println!("{}", set.len());
}
