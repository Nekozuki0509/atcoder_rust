#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [usize;n],
        mut b: [usize;n-1]
    }

    a.sort_by(|a, b| b.cmp(a));
    b.sort_by(|a, b| b.cmp(a));
    let mut over = -1isize;
    let mut now = 0;

    for i in a {
        if  b.len() != now && b[now] >= i {
            now += 1;
        } else if over == -1 {
            over = i as isize;
        } else {
            println!("-1");
            return;
        }
    }

    println!("{}", over);
}
