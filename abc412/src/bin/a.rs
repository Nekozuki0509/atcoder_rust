#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    let mut an = 0;
    for _ in 0..n {
        input! {a: usize, b: usize}

        if a < b {
            an += 1;
        }
    }

    println!("{}", an);
}
