#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        (n, mut t): (usize, isize),
        a: [isize;n]
    }

    t %= a.iter().sum::<isize>();
    for (i, v) in a.iter().enumerate() {
        t -= v;
        if t < 0 {
            println!("{} {}", i+1, v + t);
            return;
        }
    }
}
