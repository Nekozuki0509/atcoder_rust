#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

fn to_p(c: &char) -> (isize, isize) {
    match c {
                'U' => (-1, 0),
                'D' => (1, 0),
                'L' => (0, -1),
                _ => (0, 1),
    }
}

#[fastout]
fn main() {
    input! {
        (tr, tc, ar, ac): (isize, isize, isize, isize),
        (n, m, l): (usize, usize, usize),
        t: [(char, isize);m],
        mut a: [(char, isize);l]
    }

    let mut now = (tr - ar, tc - ac);
    let mut an = 0;
    let mut ai = 0;
    for (c, mut i) in t {
        let tm = to_p(&c);
        while i - a[ai].1 > 0 {
            //dbg!(now);
            let am = to_p(&a[ai].0);
            for _ in 0..a[ai].1 {
                now = (now.0 + tm.0 - am.0, now.1 + tm.1 - am.1);
                if now.0 == 0 && now.1 == 0 {
                    an += 1;
                }
            }
            i -= a[ai].1;
            ai += 1;
        }

        //dbg!(now);
        let am = to_p(&a[ai].0);
        for _ in 0..i {
            now = (now.0 + tm.0 - am.0, now.1 + tm.1 - am.1);
            if now.0 == 0 && now.1 == 0 {
                an += 1;
            }
        }
        a[ai].1 -= i;
        if a[ai].1 == 0 {
            ai += 1;
        }
    }

    println!("{}", an);
}
