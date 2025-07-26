#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize;n],
        x: usize
    }

    let an = if a.contains(&x) {
        "Yes"
    } else {
        "No"
    };

    println!("{}", an);
}
