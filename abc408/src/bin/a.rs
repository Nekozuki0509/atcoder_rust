#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        (n, s): (usize, f32),
        t: [f32;n]
    }

    let mut now = 0.;

    for i in t {
        if i - now <= s+0.5 {
            now = i;
        } else {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
