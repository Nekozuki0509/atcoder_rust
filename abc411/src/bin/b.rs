#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        n: usize,
        d: [usize;n-1]
    }

    let mut vec = vec![0];
    for (i, v) in d.iter().enumerate() {
        vec.push(vec[i] + v);
    }

    for i in 0..n {
        for j in i+1..n {
            print!("{} ", vec[j] - vec[i]);
        }
        println!();
    }
}
