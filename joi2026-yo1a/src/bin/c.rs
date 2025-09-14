#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize;n],
        b: [usize;n],
    }

    for (&a, b) in a.iter().zip(b) {
        println!("{}\n{}", a, b);
    }
}
