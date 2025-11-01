#[allow(unused_imports)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        s: usize,
    }

    if 200 <= s && s <= 299 {
        println!("Success");
    } else {
        println!("Failure");
    }
    
}
