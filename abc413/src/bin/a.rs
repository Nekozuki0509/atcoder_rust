#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        (n, m): (usize, usize),
        a: [usize;n]
    }

    let an = if a.iter().sum::<usize>() <= m {
        "Yes"
    } else {
        "No"
    };

    println!("{}", an);
}
