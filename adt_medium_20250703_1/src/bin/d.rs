use std::collections::BTreeSet;

#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        (n, m): (usize, usize),
        a: [[usize;n];m]
    }

    let mut set = BTreeSet::new();
    for i in a {
        for (j, v) in i.iter().enumerate() {
            if j==0 {
                continue;
            }

            set.insert((*v.min(&i[j-1]), *v.max(&i[j-1])));
        }
    }

    let mut an = 0usize;
    for i in 1..=n {
        for j in i+1..=n {
            if !set.contains(&(i, j)) {
                an += 1;
            }
        }
    }

    println!("{}", an);
}
