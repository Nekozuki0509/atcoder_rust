#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        (n, q): (usize, usize),
        a: [usize;n]
    }

    let mut vec = vec![0];
    for (i, &v) in a.iter().enumerate() {
        vec.push(vec[i] + v);
    }

    let mut now = 0;
    for _ in 0..q {
        input! {t: u8}
        match t {
            1 => {
                input! {c: usize}
                now += c;
                now %= n;
            },
            _ => {
                input! {l: usize, r: usize}
                let mut l = (l+now)%n;
                if l == 0 {
                    l = n;
                }
                let mut r = (r+now)%n;
                if r == 0 {
                    r = n;
                }
                if l <= r {
                    println!("{}", vec[r] - vec[l-1]);
                } else {
                    //dbg!(now, l, r);
                    println!("{}", vec[r] + vec[n] - vec[l-1])
                }
            }
        }
    }
}
