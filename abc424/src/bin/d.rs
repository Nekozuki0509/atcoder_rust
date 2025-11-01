#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        input! {
            (h, w): (usize, usize),
            s: [Chars;h]
        }

        let mut c = vec![vec![true;w];h];
        let mut ans = 0;
        for i in 1..h {
            for j in 1..w {
                if  c[i][j] && '#'.eq(&s[i][j]) && 
                    c[i][j-1] && '#'.eq(&s[i][j-1]) &&
                    c[i-1][j] && '#'.eq(&s[i-1][j]) &&
                    c[i-1][j-1] && '#'.eq(&s[i-1][j-1]) {
                    
                    ans += 1;
                    c[i][j] = false;
                }
            }
        }

        let mut c = vec![vec![true;w];h];
        let mut ans2 = 0;
        for i in 0..h-1 {
            for j in 1..w {
                if  c[i][j] && '#'.eq(&s[i][j]) && 
                    c[i][j-1] && '#'.eq(&s[i][j-1]) &&
                    c[i+1][j] && '#'.eq(&s[i+1][j]) &&
                    c[i+1][j-1] && '#'.eq(&s[i+1][j-1]) {
                    
                    ans2 += 1;
                    c[i][j] = false;
                }
            }
        }

        let mut c = vec![vec![true;w];h];
        let mut ans3 = 0;
        for i in 1..h {
            for j in 0..w-1 {
                if  c[i][j] && '#'.eq(&s[i][j]) && 
                    c[i][j+1] && '#'.eq(&s[i][j+1]) &&
                    c[i-1][j] && '#'.eq(&s[i-1][j]) &&
                    c[i-1][j+1] && '#'.eq(&s[i-1][j+1]) {
                    
                    ans3 += 1;
                    c[i][j] = false;
                }
            }
        }

        let mut c = vec![vec![true;w];h];
        let mut ans4 = 0;
        for i in 0..h-1 {
            for j in 0..w-1 {
                if  c[i][j] && '#'.eq(&s[i][j]) && 
                    c[i][j+1] && '#'.eq(&s[i][j+1]) &&
                    c[i+1][j] && '#'.eq(&s[i+1][j]) &&
                    c[i+1][j+1] && '#'.eq(&s[i+1][j+1]) {
                    
                    ans4 += 1;
                    c[i][j] = false;
                }
            }
        }

        println!("{}", (ans.min(ans2)).min(ans3.min(ans4)));
    }
}
