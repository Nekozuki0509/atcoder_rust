#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        p: Chars,
        l: usize
    }

    println!("{}", if p.len() >= l { "Yes" } else { "No" })
}
