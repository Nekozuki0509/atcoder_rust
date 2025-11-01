#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        (n, r): (usize, usize),
        l: [u8;n]
    }

    let mut flag = false;
    let mut cnt = 0;
    let mut lcnt = 0;
    for (i, &v) in l.iter().enumerate() {
        if v == 0 {
            cnt += 1;
            flag = true;
            cnt += lcnt * 2;
            lcnt = 0;
        } else if i == r && !flag {
            flag = true;
            lcnt += 1;
        } else if i + 1 == r && flag {
            cnt += (lcnt + 1) * 2;
            lcnt = 0;
        }
        else {
            if flag {
                lcnt += 1
            }
        }
    }

    println!("{}", cnt);
}
