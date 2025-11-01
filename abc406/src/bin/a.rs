#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        (a, b, c, d): (usize, usize, usize, usize)
    }
    
    if c < a {
      println!("Yes");
    } else if c == a && d < b {
      println!("Yes");
    } else {
      println!("No");
    }

    
}