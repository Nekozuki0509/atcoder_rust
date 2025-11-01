#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        s: Chars,
    }

    for i in 97..=122u8 {
        if !s.contains(&(i as char)) {
            println!("{}", i as char);
            return;
        }
    } 
}
