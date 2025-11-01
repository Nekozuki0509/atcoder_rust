#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize;n],
        k: usize
    }

    println!("{}", a.iter().filter(|&x| x >= &k).count());

    
}
