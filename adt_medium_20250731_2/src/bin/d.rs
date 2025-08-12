use std::collections::BTreeSet;

use amplify::confinement::Collection;
#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        s: Chars,
    }

    let mut set = BTreeSet::new();
    let mut upper = false;
    let mut lower = false;
    for i in &s {
        set.push(i);
        if i.is_uppercase() {
            upper = true;
        } else {
            lower = true;
        }
    }

    let an = if set.len() == s.len() && upper && lower {
        "Yes"
    } else {
        "No"
    };

    println!("{}", an);
}
