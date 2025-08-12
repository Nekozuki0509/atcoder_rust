#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        (n, s): (usize, usize),
        t: [(usize, usize);n]
    }

    for i in 0..(1usize << n) {
        let mut sum = 0usize;
        let mut str = String::new();
        for j in 1..=n {
            if (i & (1usize << (j-1))) != 0 {
                sum += t[j-1].0;
                str.push('T');
            } else {
                sum += t[j-1].1;
                str.push('H');
            }
        }

        if sum == s {
            println!("Yes");
            println!("{}", str);
            return;
        }
    }

    println!("No");
}
