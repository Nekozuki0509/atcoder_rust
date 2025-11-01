#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        s: Chars,
    }

    let mut an: f64 = 0.;
    for i in 0..s.len() {
        for j in i+2..s.len() {
            if s[i].eq(&'t') && s[j].eq(&'t') {
                let count = s[i..=j].iter().fold(0, |x, c| if c.eq(&'t') {x+1} else {x});
                let filling_rate = (count - 2) as f64 / (j-i-1) as f64;
                if count >= 2 && an < filling_rate {
                    an = filling_rate;
                }
            }
        }
    }

    println!("{}", an);
}
