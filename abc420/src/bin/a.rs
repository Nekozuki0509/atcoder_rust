#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        (x, y): (usize, usize)
    }

    println!("{}", if (x + y) % 12 == 0 {12} else {(x+y) % 12} )
}
