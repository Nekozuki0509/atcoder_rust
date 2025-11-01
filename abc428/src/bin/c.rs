use std::collections::VecDeque;

#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        q: usize
    }

    let mut now = 0isize;
    let mut que = VecDeque::new();
    let mut ps = VecDeque::new();
    for _ in 0..q {
        input! {t: u8}
        match t {
            1 => {
                input! {c: char}
                match c {
                    '(' => now += 1,
                    _ => {
                        if now == 0 {
                            ps.push_back(que.len());
                        }
                        now -= 1;
                    }
                }
                
                que.push_back(c);
            },
            _ => {
                if let Some(x) = que.pop_back() {
                    match x {
                        '(' => now -= 1,
                        _ => {
                            now += 1;
                            if now == 0 {
                                if let Some(p) = ps.pop_back() {
                                    if p != que.len() {
                                        ps.push_back(p);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        
        if ps.is_empty() && now == 0 {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
