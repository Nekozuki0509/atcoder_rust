use std::collections::VecDeque;

use bitvec::vec;
#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        q: usize,
    }

    let mut que = vec![];
    let mut now = 0usize;
    let mut den = 0usize;
    for _ in 0..q {
        input! {t: u8}
        match t {
            1 => {
                input! {l: usize}
                que.push(now);
                now += l;
            },
            2 => {
                den += 1;
            },
            3 => {
                input! {k: usize}
                println!("{}", que[den+k-1] - que[den]);
            },
            _ => unreachable!()
        }
    }
}
