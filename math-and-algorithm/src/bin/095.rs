use itertools::enumerate;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
    }

    let mut one = vec![];
    let mut two = vec![];
    let mut to = vec![(0, 0)];
    for _ in 0..n {
        input! {c: u8, p: usize}

        match c {
            1 => {
                one.push(p);
            }
            _ => {
                two.push(p);
            }
        }
        to.push((one.len(), two.len()));
    }

    let mut none = vec![0];
    let mut ntwo = vec![0];
    for (i, &v) in one.iter().enumerate() {
        none.push(none[i] + v);
    }
    for (i, &v) in two.iter().enumerate() {
        ntwo.push(ntwo[i] + v);
    }

    input! {q: usize}
    for _ in 0..q {
        input! {l: Usize1, r: usize}
        println!(
            "{} {}",
            none[to[r].0] - none[to[l].0],
            ntwo[to[r].1] - ntwo[to[l].1]
        );
    }
}
