#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        n: usize
    }

    let mut an = String::new();
    for _ in 0..n {
        input! {c: String, l: usize}
        if an.len() + l > 100 {
            println!("Too Long");
            return;
        }
        an.push_str(&vec![c; l].concat());
    }

    println!("{}", an);
}
