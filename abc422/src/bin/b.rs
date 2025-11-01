#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        (h, w): (usize, usize),
        s: [Chars;h]
    }

    let mut vec = vec![vec![false;w+2];h+2];
    for (i, v) in s.iter().enumerate() {
        for (j, w) in v.iter().enumerate() {
            if '#'.eq(w) {
                vec[i+1][j+1] = true;
            }
        }
    }

    for (i, v) in vec.iter().enumerate() {
        for (j, &w) in v.iter().enumerate() {
            if w {
                let mut cnt = 0;
                if vec[i-1][j] {
                    cnt += 1;
                }
                if vec[i+1][j] {
                    cnt += 1;
                }
                if vec[i][j-1] {
                    cnt += 1;
                }
                if vec[i][j+1] {
                    cnt += 1;
                }

                if cnt != 2 && cnt != 4 {
                    println!("No");
                    return;
                }
            }
        }
    }

    println!("Yes");
}
