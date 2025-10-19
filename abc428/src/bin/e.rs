#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    let mut g = vec![vec![];n];
    for _ in 1..n {
        input! {a: Usize1, b: Usize1}
        g[a].push(b);
        g[b].push(a);
    }

    let mut dist = vec![-1;n];
}
