#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        (n, k): (usize, usize),
        a: [usize;n]
    }

    let mut now = 1usize;
    for i in a {
        let x = now.checked_mul(i).unwrap_or(1);
        if x.to_string().len() <= k {
            now = x;
        } else {
            now = 1;
        }
    }

    println!("{}", now);
}
