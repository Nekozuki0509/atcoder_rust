#![allow(unaused_imports, dead_code, non_snake_case)]
#![allow(unaused_variables, unaused_assignments)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1, Isize1}};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

#[fastout]
fn main() {
    input! {
        (n, a, b): (usize, usize, usize),
        s: Chars
    }

    let mut que = VecDeque::new();
    for i in s {
        match i {
            'a' => {
                if let Some((c, v)) = que.pop_back() {
                    if c == 0 {
                        que.push_back((c, v+1));
                    } else {
                        que.push_back((c, v));
                        que.push_back((0, 1));
                    }
                } else {
                    que.push_back((0, 1));
                }
            },
            _ => {
                if let Some((c, v)) = que.pop_back() {
                    if c == 1 {
                        que.push_back((c, v+1));
                    } else {
                        que.push_back((c, v));
                        que.push_back((1, 1));
                    }
                } else {
                    que.push_back((1, 1));
                }
            }
        }
    }
    
    let mut flag = true;
    let mut last = 0;
    let mut ans = 0;
    let mut lasta = 0;
    let mut lastb = 0;
    let mut nowa = 0;
    let mut nowb = 0;
    let mut aused = VecDeque::new();
    let mut bused = VecDeque::new();
    while let Some((c, v)) = que.pop_front() {
        if c == 0 {
            if flag {
                last = 0;
                flag = false;
            }

            nowa += v;
            if nowa >= a {
                ans += nowa - a + 1;
                //dbg!(nowa - a + 1, ans);
            }

            aused.push_back(v);
        } else {
            if flag {
                last = 1;
                flag = false;
            }

            nowb += v;
            if nowa >= a && nowb < b {
                ans += v;
            } else if nowb >= b {
                if nowa >= a {
                    ans += nowb - b;
                    //dbg!(nowb - b, ans);
                    if last == 0 {
                        ans += (nowa - a).min(lasta);
                    }
                    //dbg!((nowa - a).min(lasta), ans);
                }

                nowa -= lasta;
                if nowa >= a {
                    ans += lastb;
                    //dbg!(lastb, ans);
                }
                lastb -= nowb - b;
                nowb -= b - 1;
                if lastb > 0 {
                    last = 1;
                } else {
                    last = 0;
                }
                
                if let Some(x) = aused.pop_front() {
                    lasta = x;
                }
            }
            bused.push_back(v);
        }
    }

    println!("{}", ans);
}
