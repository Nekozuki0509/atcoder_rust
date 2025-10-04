#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        n: u32,
    }

    let mut ans = 0;
    for i in 1..=n {
        ans += (-1isize).pow(i) * i.pow(3) as isize;
    }

    println!("{}", ans)
}
