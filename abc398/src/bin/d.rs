use std::collections::VecDeque;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        r: isize,
        c: isize,
        s: Chars
    }

    let mut rv = VecDeque::new();
    for _ in 1..=2*n+1 {
        let mut cv = VecDeque::new();
        for _ in 1..2*n+1 {
            cv.push_back(false);
        }
        rv.push_back(cv);
    }

    rv.get_mut(n).unwrap()[n] = true;

    for w in s {
        if w.eq(&'N') {
            for cve in rv.iter_mut() {
                cve.remove(2*n);
                cve.push_front(false);
            }
        } else if w.eq(&'W') {
            rv.remove(0);
            let mut cv = VecDeque::new();
            for _ in 1..2*n+1 {
                cv.push_back(false);
            }
            rv.push_back(cv);
        } else if w.eq(&'S') {
            for cve in rv.iter_mut() {
                cve.remove(0);
                cve.push_back(false);
            }
        } else if w.eq(&'E') {
            rv.remove(2*n);
            let mut cv = VecDeque::new();
            for _ in 1..2*n+1 {
                cv.push_back(false);
            }
            rv.push_front(cv);
        }

        rv.get_mut(n).unwrap()[n] = true;

        if *rv.get((r + (n as isize)) as usize).unwrap().get((c + (n as isize)) as usize).unwrap() {
            print!("1");
        } else {
            print!("0");
        }
    }

    println!("");
    rv.iter().for_each(|x| println!("{:?}", x));
}
