#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        t: usize
    }

    'q: for _ in 0..t {
        input! {n: usize, mut s: [usize;n]}
        let mut an = 1usize;
        let ld = s[0];
        let mut now = *s.last().unwrap();
        let mut last = now;
        s.sort();
        s.reverse();
        for i in s {
            //dbg!(&i);
            if i * 2 < now {
                if now == last {
                    println!("-1");
                    continue 'q;
                }

                now = last;
                if i * 2 < now {
                    println!("-1");
                    continue 'q;
                } else {
                    if i == ld {
                        an += 2;
                        break;
                    }
                    last = i;
                }
                an += 1;
                //dbg!(&last, &now);
            } else {
                if i == ld {
                    an += 1;
                    break;
                }
                last = i;
                //dbg!(&last);
            }
            //dbg!(&an);
        }

        println!("{}", an);
    }
}
