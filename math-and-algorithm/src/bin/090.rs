use std::collections::BTreeSet;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        (n, b): (usize, usize),
    }

    let mut set = BTreeSet::new();
    set.insert(0);
    for i in (1..10).combinations_with_replacement(11) {
        set.insert(i.iter().product::<usize>());
    }

    let mut ans = 0usize;
    for i in set {
        let t = i + b;
        if t > n {
            continue;
        }

        let mut prob = 1;
        let mut now = t;
        loop {
            prob *= now % 10;
            now /= 10;
            if now == 0 || prob == 0 {
                break;
            }
        }

        if prob == i {
            ans += 1;
        }
    }

    println!("{}", ans);
}
