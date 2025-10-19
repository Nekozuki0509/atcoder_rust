use std::mem::swap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        v: [(isize, isize);n],
        p: (isize, isize)
    }

    let mut now = 0;
    for i in 0..n {
        let mut f = (v[i].0 - p.0, v[i].1 - p.1);
        let mut s = (v[(i + 1) % n].0 - p.0, v[(i + 1) % n].1 - p.1);
        if s.1 < f.1 {
            swap(&mut f, &mut s);
        }

        if f.1 <= 0 && 0 < s.1 && f.0 * s.1 - s.0 * f.1 < 0 {
            now ^= 1;
        }
    }

    println!("{}", ["OUTSIDE", "INSIDE"][now]);
}
