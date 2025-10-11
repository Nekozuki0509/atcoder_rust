#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    let mut v = vec![1];
    for i in 0..n {
        let mut s = 0;
        for i in v[i].to_string().chars() {
            s += i.to_string().parse::<usize>().unwrap();
        }
        v.push(v[i] + s);
    }

    println!("{}", v[n-1]);
}
