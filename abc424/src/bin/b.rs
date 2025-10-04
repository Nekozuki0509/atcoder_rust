use std::collections::BTreeSet;

#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        (n, m, k): (usize, usize, usize)
    }

    let mut t = vec![0;n];
    let mut vec = vec![true;n];
    let mut ans = vec![];
    for _ in 0..k {
        input! {a: Usize1, _: usize}
        t[a] += 1;
        if vec[a] && t[a] == m {
            ans.push((a+1).to_string());
            vec[a] = false;
        }
    }

    println!("{}", ans.join(" "));
}
