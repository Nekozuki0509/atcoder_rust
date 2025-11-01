use std::collections::BTreeMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize;n]
    }

    let mut an = 0;
    for b in 0..a.len() {
        for c in b+1..a.len() {
            for d in c+1..a.len() {
                for e in d+1..a.len() {
                    for f in e+1..a.len()                                                                                                                                                    {
                        if a[b] + a[c] + a[d] + a[e] + a[f] == 1000 {
                            an += 1;
                        }
                    }
                }
            }
        }
    }

    println!("{}", an);
}
