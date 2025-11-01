#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        (n, q): (usize, usize),
        mut a: [isize;n],
        mut b: [isize;n]
    }

    let mut now = vec![];
    let mut sum = 0;
    for i in 0..n {
        if a[i] <= b[i] {
            now.push(('a', a[i]));
            sum += a[i];
        } else {
            now.push(('b', b[i]));
            sum += b[i];
        }
    }

    //dbg!(sum);

    for _ in 0..q {
        input! {c: char, x: usize, y: isize}
        match c {
            'A' => {
                //dbg!(y, b[x-1]);
                if y <= b[x-1] {
                    sum += y - now[x-1].1;
                    now[x-1].1 = y;
                } else {
                    sum += b[x-1] - now[x-1].1;
                    now[x-1] = ('b', b[x-1]);
                }
                a[x-1] = y;
                //dbg!(&now, sum);
                println!("{}", sum);
            },
            _ => {
                if y <= a[x-1] {
                    sum += y - now[x-1].1;
                    now[x-1].1 = y;
                } else {
                    sum += a[x-1] - now[x-1].1;
                    now[x-1] = ('a', a[x-1]);
                }
                b[x-1] = y;
                //dbg!(&now);
                println!("{}", sum);
            }
        }
    }
}
