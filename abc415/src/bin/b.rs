#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        s: Chars
    }

    let mut last = 0usize;
    for (i, c) in s.iter().enumerate() {
        if c.eq(&'#') {
            if last != 0 {
                println!("{},{}", last, i+1);
                last = 0;
            } else {
                last = i+1;
            }
        }
    }
}
