#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        t: usize,
    }

    let mut v = vec![0, 1, 4, 5, 6, 9];
    for _ in 0..t {
        input! {c: usize, d: usize}
    }
}
