#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        (n, m): (usize, usize),
        s: String,
        t: Chars
    }

    let head = t[..n].iter().collect::<String>();
    let tale = t[m-n..].iter().collect::<String>();

    let an = if head.eq(&s) && tale.eq(&s) {
        0
    } else if head.eq(&s) {
        1
    } else if tale.eq(&s) {
        2
    } else {
        3
    };

    println!("{}", an);
}
