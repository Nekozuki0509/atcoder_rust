#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        r: usize,
        x: usize
    }

    if x == 1 && 1600 <= r && r <= 2999 {
        println!("Yes");
    } else if x == 2 && 1200 <= r && r <= 2399 {
        println!("Yes");
    } else {
        println!("No");
    }
}
