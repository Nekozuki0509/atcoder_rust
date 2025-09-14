use num::abs;
use proconio::input;

fn main() {
    input! {
        (n, k): (isize, isize),
    }

    let mut an = 0isize;
    for a in 1..=n {
        for b in 1.max(a-(k-1))..=n.min(a+(k-1)) {
            for c in 1.max(a-(k-1))..=n.min(a+(k-1)) {
                if abs(b - c) < k {
                    an += 1;
                }
            }
        }
    }
    
    println!("{}", n.pow(3) - an);
}
