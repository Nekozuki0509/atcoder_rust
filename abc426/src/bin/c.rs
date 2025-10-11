#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        (n, q): (usize, usize)
    }

    let mut vec = vec![1usize;n+2];
    let mut min = 0;
    vec[0] = 0;
    for _ in 0..q {
        input! {x: usize, y: usize}
        if x < min+1 {
            println!("0");
            continue;
        }
        
        let mut an = 0;
        for v in vec[min+1..=x].iter() {
            an += v;
        }

        println!("{}", an);
        vec[y] += an;
        min = x;
    }
}
