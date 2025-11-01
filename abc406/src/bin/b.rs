#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [isize; n]
    }
    
    let mut now: isize = 1;

    for i in a {
        if let Some(x) = now.checked_mul(i) {
            if x.to_string().len() > k {
                now = 1;
            } else {
                now = x;
            }
        } else {
            now = 1;
        }

        dbg!(now);
    }

    println!("{}", now);
}
