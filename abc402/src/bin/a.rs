#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        s: Chars,
    }

    let mut an = s.iter().filter(|x| x.is_uppercase()).collect::<String>();
    
    println!("{}", an);
}
