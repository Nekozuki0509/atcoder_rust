use itertools::Itertools;
#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        (n, q): (usize, usize),
        x: [usize;q]
    }

    let mut vec = vec![0;n];
    for i in x {
        if i != 0 {
            vec[i-1] += 1;
            print!("{} ", i);
        } else {
            let min = vec.iter().position_min().unwrap();
            vec[min] += 1;
            print!("{} ", min+1);
        }
    }
}
