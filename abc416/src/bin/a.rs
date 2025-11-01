#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        (n, l, r): (usize, usize, usize),
        s: Chars
    }

    for i in s[l-1..r].iter() {
        if !i.eq(&'o') {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
