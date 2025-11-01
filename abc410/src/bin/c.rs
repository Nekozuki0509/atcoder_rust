#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        (n, q): (usize, usize)
    }

    let mut a = vec![];
    for i in 1..=n {
        a.push(i);
    }

    let mut off = 0;
    for _ in 0..q {
        input! {t: usize}
        match t {
            1 => {
                input! {(p, x): (usize, usize)}
                a[(off+p-1)%n] = x;
            },
            2 => {
                input! {p: usize}
                println!("{}", a[(off+p-1)%n]);
            },
            _ => {
                input! {k: usize}
                off += k;
                off %= n;
                //dbg!(&a);
            }
        };

        //dbg!(&a);
    }
}
