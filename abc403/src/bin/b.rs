#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

use regex::Regex;

#[fastout]
fn main() {
    input! {
        mut t: String,
        u: String
    }

    t = t.replace("?", ".");

    let mut an = "No";
    for i in 0..=t.len()-u.len() {
        let temp = &t[i..i+u.len()];
        let re = Regex::new(temp).unwrap();
        //println!("{}", temp);
        if re.is_match(&u) {
            an = "Yes";
            break;
        }
    }
    println!("{}", an);
}
