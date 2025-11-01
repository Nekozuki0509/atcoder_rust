#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};
use regex::Regex;

#[fastout]
fn main() {
    input! {
        s: String,
    }

    let mut an = 0;
    for i in 0..49 {
        let re = Regex::new(&format!("A.{{{}}}B.{{{}}}C", i, i)).unwrap();
        for i in re.find_iter(&s) {
            println!("{}", i.as_str());
            an += 1;
        }
        //an += re.find_iter(&s).count();
    }
    println!("{}", an);
}
