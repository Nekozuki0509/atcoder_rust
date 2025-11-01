#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        (n, m): (usize, usize),
        (s, t): (Chars, Chars)
    }

    let mut vec = vec![0isize;n+1];
    for _ in 0..m {
        input! {l: usize, r: usize}
        vec[l-1] += 1;
        vec[r] -= 1;
    }

    //dbg!(&vec);

    let mut ans = String::new();
    let mut last = 0;
    for (i, &v) in vec[..n].iter().enumerate() {
        if (last+v) % 2 == 0 {
            ans.push(s[i]);
            //dbg!("s");
        } else {
            ans.push(t[i]);
            //dbg!("t");
        }
        last += v;
    }

    println!("{}", ans);
}
