#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        mut s: Chars,
    }

    let mut an = 0;

    for i in s.iter().rev() {
        let now: isize = (i.to_string().parse::<isize>().unwrap() - (an % 10) + 10) % 10;
        an += now;
    }

    println!("{}", an as usize + s.len());
}
