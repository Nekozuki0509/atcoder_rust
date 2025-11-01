#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        (n, m): (usize, usize),
        s: [Chars;n],
        t: [String;m]
    }

    let s = s.iter().map(|x| x[3..].iter().collect::<String>()).collect::<Vec<String>>();
    let mut an = 0;
    for i in s {
        if t.contains(&i) {
            an += 1usize;
        }
    }

    println!("{}", an);
}
