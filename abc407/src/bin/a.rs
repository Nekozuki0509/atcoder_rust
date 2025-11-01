#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        (a, b): (f32, f32)
    }

    println!("{}", (a / b).round());
    
}
