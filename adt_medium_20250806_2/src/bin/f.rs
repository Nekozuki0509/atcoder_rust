#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        (n, k): (usize, usize),
        s: Chars
    }

    let mut vec: Vec<(usize, char)> = vec![];
    let mut count = 0;
    for i in s {
        if let Some(x) = vec.last_mut() {
            if i.eq(&x.1) {
                x.0 += 1;
            } else {
                vec.push((1, i));
            }
        } else {
            vec.push((1, i));
            count = if i.eq(&'0') {2 * k - 1} else {2 * k - 2};
        }
    }

    let num = vec[count].0;

    let mut now = 1usize;
    for i in vec {
        if now == count - 1 {
            print!("{}", i.1.to_string().repeat(i.0));
            print!("{}", "1".repeat(num));
            now += 1;
        } else if now == count + 1 {
            now += 1;
            continue;
        } else {
            now += 1;
            print!("{}", i.1.to_string().repeat(i.0));
        }
    }
}
