use std::collections::VecDeque;

#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        q: usize,
    }

    //(x, c, a)
    let mut vec = VecDeque::new();
    for _ in 0..q {
        input! {t: u8}
        match t {
            1 => {
                input! {c: usize, x: usize}
                vec.push_back((x, c));
                //dbg!(&vec);
            },
            2 => {
                input! {mut k: usize}
                let mut an = 0;
                while let Some((v, l)) = vec.pop_front() {
                    if k < l {
                        an += v * k;
                        vec.push_front((v, l-k));
                        break;
                    } else {
                        k -= l;
                        an += v * l;
                    }
                }
                println!("{}", an);
            },
            _ => unreachable!()
        }
    }
}
