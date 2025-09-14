#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        s: Chars,
    }

    let i: i32 = s[0].to_string().parse().unwrap();
    let j: i32 = s[2].to_string().parse().unwrap();
    let j = if j < 8 {j+1} else {1};
    let i = if j == 1 {i + 1} else {i};

    println!("{}-{}", i, j)
}
