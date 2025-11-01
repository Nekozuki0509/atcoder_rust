#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        s: Chars,
    }

    let mut ans = String::new();
    for (i, &v) in s.iter().enumerate() {
        if i == (s.len()+1) / 2 - 1 {
            continue;
        }

        ans.push(v);
    }

    println!("{}", ans);
}
