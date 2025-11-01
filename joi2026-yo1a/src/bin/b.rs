#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        (a, b): (usize, usize)
    }

    println!("{}", if a > b * 3 {1} else {0});
}
