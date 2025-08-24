#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        s: String,
    }

    if s.eq(&"red") {
        println!("SSS");
    } else if s.eq(&"blue") {
        println!("FFF");
    } else if s.eq(&"green") {
        println!("MMM");
    } else {
        println!("Unknown");
    }
}
