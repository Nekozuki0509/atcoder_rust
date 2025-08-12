#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: Chars
    }

    println!("{}", if n >= 3 && s[n-3..].iter().collect::<String>().eq(&"tea") {"Yes"} else {"No"})
}
