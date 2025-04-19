use std::collections::{BTreeMap, BTreeSet};

#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        (n, m): (usize, usize),
    }

    let mut d = vec![];
    for _ in 0..m {
        input! {
            k: usize,
            a: [usize; k]
        }
        d.push(a);
    }

    input! {mut b: [usize; n]}

    let mut map = vec![m];
    b.reverse();
    let mut set = BTreeSet::new();
    for i in b {
        for j in 0..d.len() {
            if d[j].contains(&i) {
                set.push(j);
            }
        }

        map.push(m-set.len());

        //println!("t={}, map={:?}", t, map);
    }

    map.reverse();
    println!("{}", map[1..].iter().map(|x| x.to_string()).collect::<Vec<String>>().join("\n"));
}
