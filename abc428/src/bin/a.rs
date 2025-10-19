#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        (s, a, b, x): (usize, usize, usize, usize),
    }

    let mut ans = 0;
    ans += (x / (a+b)) * s * a;
    ans += a.min(x % (a + b)) * s;

    println!("{}", ans);
}