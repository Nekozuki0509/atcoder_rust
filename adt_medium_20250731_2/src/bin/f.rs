#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize;n]
    }

    let mut vec= vec![0usize;2*n+1];
    for (i, v) in a.iter().enumerate() {
        vec[2*(i+1)-1] = vec[*v-1] + 1;
        vec[2*(i+1)] = vec[*v-1] + 1;
    }

    for i in vec {
        println!("{}", i);
    }
}
