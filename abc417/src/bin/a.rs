#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        (n, a, b): (usize, usize, usize),
        s: Chars
    }

    let mut an = String::new();
    for i in s[a..s.len()-b].iter() {
        an.push(*i);
    }

    println!("{}", an);
}
