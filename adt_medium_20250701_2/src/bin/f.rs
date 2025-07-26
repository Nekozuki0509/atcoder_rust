use std::collections::BTreeSet;

#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [usize;n]
    }

    let mut no = vec![];
    for i in 1..=n {
        if !a.contains(&i) {
            no.push(i);
        }
    }

    let last = *no.last().unwrap_or(&(n+1));
    a.sort();
    a.dedup();
    let mut can_sell = n-a.len();
    let buy = can_sell / 2;
    if buy > no.len() {
        no.clear();
    } else {
        no.drain(..buy);
    }
    can_sell %= 2;

    loop {
        if no.is_empty() {
            break;
        }

        if a.len() > 1 && no[0] < a[a.len()-2] {
            a.remove(a.len()-1);
            a.remove(a.len()-1);
            no.remove(0);
        } else if a.len() > 0 && no[0] < a[a.len()-1] && can_sell > 0 {
            a.remove(a.len()-1);
            can_sell -= 1;
            no.remove(0);
        } else {
            break;
        }
    }

    println!("{}", no.first().unwrap_or(&last)-1);
}
