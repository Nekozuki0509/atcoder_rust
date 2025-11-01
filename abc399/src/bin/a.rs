#[allow(unused_imports)]
use proconio::{input, marker::{Bytes, Chars, Usize1}};

#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        s: Chars,
        t: Chars
    }

    let mut an = 0;
    for i in 0..n {
        if !s[i].eq(&t[i]) {
            an += 1;
        }
    }
    
    println!("{}", an);
}
