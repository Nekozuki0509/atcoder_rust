#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        (a, b, c): (usize, usize, usize)
    }

    if a == b || a == c || b == c {
        println!("Yes");
    } else {
        println!("No");
    }
}
