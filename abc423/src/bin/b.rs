#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        n: usize,
        l: [u8;n]
    }

    let min = l.iter().position(|&x| x == 1).unwrap_or(0);
    let max = n - l.iter().rev().position(|&x| x == 1).unwrap_or(n-1) - 1;

    //dbg!(min, max);

    println!("{}", max - min);
}
