#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        (n, m): (usize, usize),
        d: [(usize, usize);m]
    }

    let lows = d.iter().map(|(_, l)| *l).collect::<Vec<usize>>();
    let mut saikyo = -1isize;
    for i in 1..=n {
        if !lows.contains(&i) {
            if saikyo != -1 {
                println!("-1");
                return;
            } else {
                saikyo = i as isize;
            }
        }
    }

    println!("{}", saikyo);
}
