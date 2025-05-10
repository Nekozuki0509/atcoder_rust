#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    
    
    println!("{}", a.iter().enumerate().filter(|(i, v)| i % 2 == 0).map(|(i, v)| v).sum::<usize>());
}
