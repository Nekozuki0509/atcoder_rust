use std::collections::BTreeSet;

#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        (n, q): (usize, usize)
    }

    let mut vec = vec![false;n+2];
    let mut now = 0;
    for _ in 0..q {
        input! {i: usize}
        if !vec[i-1] && !vec[i] && !vec[i+1] {
            now += 1;
        } else if !vec[i-1] && !vec[i] && vec[i+1] {
        } else if !vec[i-1] && vec[i] && !vec[i+1] {
            now -= 1;
        } else if !vec[i-1] && vec[i] && vec[i+1] {
        } else if vec[i-1] && !vec[i] && !vec[i+1] {
        } else if vec[i-1] && !vec[i] && vec[i+1] {
            now -= 1;
        } else if vec[i-1] && vec[i] && !vec[i+1] {
        } else {
            now += 1;
        }

        println!("{}", now);
        vec[i] = !vec[i];
    }
}
