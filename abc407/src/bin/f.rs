#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    println!("{}", a.iter().sum());

    let mut l = vec![].join(a);

    for (i, v) in a.iter().enumerate() {
        for j in 1..a.len() {
            if i < j {
                continue;
            }
        }
    }
    
}
