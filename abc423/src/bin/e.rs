#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        (n, q): (usize, usize),
        a: [usize;n]
    }

    let mut sum = vec![0];
    for (i, &v) in a.iter().enumerate() {
        sum.push(sum[i] + v);
    }

    dbg!(&sum);

    for _ in 0..q {
        input! {l: usize, r: usize}
        let mut ans = sum[r] * (r - l);
        dbg!(ans);
        for i in l..r {
            dbg!(sum[l]);
            ans -= sum[l];
        }
        println!("{}", ans);
    }
}
