#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        a: isize,
        b: isize
    }

    println!("{}", if a < b {-1} else if a > b {1} else {0});
}
