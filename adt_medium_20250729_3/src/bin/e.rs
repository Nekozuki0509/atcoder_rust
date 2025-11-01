#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        (n, m): (usize, usize),
        s: [String;n],
        t: [String;m]
    }

    println!("Yes");

    let mut last = 0usize;
    for i in t[1..].iter() {
        let index = s[last+1..].iter().position(|x| i.eq(x)).unwrap()+last+1;
        for _ in last..index-1 {
            println!("No");
        }
        println!("Yes");
        last = index;
    }
}
