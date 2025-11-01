#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        s: Chars,
    }

    let mut sharp = true;
    let mut an = String::new();
    for i in s {
        if i.eq(&'#') {
            an.push(i);
            sharp = true;
        } else if i.eq(&'.') && sharp {
            an.push('o');
            sharp = false;
        } else {
            an.push(i);
        }
    }

    println!("{}", an);
}
