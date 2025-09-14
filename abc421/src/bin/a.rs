#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: [String;n],
        (x, y): (usize, String)
    }

    for (i, v) in s.iter().enumerate() {
        if i + 1 == x && y.eq(v) {
            println!("Yes");
            return;
        } 
    }

    println!("No");
}
