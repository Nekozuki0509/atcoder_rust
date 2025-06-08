#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        n: usize,
        t: Chars,
        a: Chars
    }

    for i in 0..n {
        if t[i].eq(&a[i]) && t[i].eq(&'o') {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
