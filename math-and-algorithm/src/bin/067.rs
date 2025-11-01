use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        (h, w): (usize, usize),
        a: [[usize;w];h]
    }

    let r = a.iter().map(|x| x.iter().sum::<usize>()).collect_vec();
    let c = a.iter().fold(vec![0usize;w], |v, x| v.iter().enumerate().map(|(i, &v)| v + x[i]).collect_vec());

    for i in 0..h {
        for j in 0..w {
            print!("{} ", r[i]+c[j]-a[i][j]);
        }
        println!();
    }
}
