#![allow(unused_imports, dead_code, non_snake_case)]
#![allow(unused_variables, unused_assignments)]
use itertools::Itertools;
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1, Isize1}};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut h: [isize;n],
        mut c: [isize;n],
        a: [[isize;n];n]
    }

    let mut last = 200;

    let mut available: Vec<usize> = vec![];
    while last > 0 {
        for i in available {
            let max = a[i].iter().enumerate().filter(|&(i, _)| h[i] > 100).max_by(|(_, &v)| v)
                .or_else(a[i].iter().enumerate().filter(|&(i, _)| h[i] > 50).max_by(|(_, &v)| v))
                .or_else(a[i].iter().enumerate().filter(|&(i, _)| h[i] > 20).max_by(|(_, &v)| v));
            if let Some(x) = max {
                
            }
        }
        loop {
            let min = h.iter().enumerate().filter(|(_, &x)| x > 0).min_by_key(|(_, &x)| x).unwrap().0 as usize;
            if min <= 100 && last > 0 {
                for _ in 0..h[min] {
                    println!("-1 {}", min);
                }
            
                h[min] = 0;
                available.push(min);
                last -= 1;
            } else {
                break;
            }

            if last == 0 {
                break;
            }
        }

        if last == 0 {
            break;
        }
        //dbg!(last, &h);
        let mut min = 0;
        let mut mind = 1 << 60;
        let mut minc = 0;
        let mut maxl = 0;
        let mut mins = -1isize;
        let mut mini = 0;
        for (ai, &s) in available.iter().enumerate() {
            for (i, &v) in h.iter().enumerate() {
                if v <= 0 || a[s][i] < 20 {
                    continue;
                }
        
                for j in 1..=c[s] {
                    if v - a[s][i] * j <= 0 && mind >= 0 {
                        min = i;
                        mins = s as isize;
                        mini = ai;
                        mind = v - a[s][i] * j;
                        minc = j;
                        maxl = c[s] - j;
                        //dbg!("a", min, mins, v, a[s][i], j, mind);
                        
                        break;
                    } else if v - a[s][i] * j <= 0 && mind <= 0 && c[s] - j > maxl {
                        min = i;
                        mins = s as isize;
                        mini = ai;
                        mind = v - a[s][i] * j;
                        minc = j;
                        maxl = c[s] - j;
                        //dbg!("b", min, mins, v, a[s][i], j, mind);

                        break;
                    } else if v - a[s][i] * j <= 0 {
                        break;
                    } else if mind > v - a[s][i] * j {
                        min = i;
                        mins = s as isize;
                        mini = ai;
                        mind = v - a[s][i] * j;
                        minc = j;
                        maxl = c[s] - j;
                        //dbg!("c", min, mins, v, a[s][i], j, mind);
                    } else if mind == v - a[s][i] * j && c[s] - j > maxl {
                        min = i;
                        mins = s as isize;
                        mini = ai;
                        minc = j;
                        maxl = c[s] - j;
                        //dbg!("d", min, mins, v, a[s][i], j, mind);
                    }
                }
            }
        }
    
        if mins != -1 {
            let mins = mins as usize;
            for _ in 0..minc {
                println!("{} {}", mins, min);
            }
            
            //dbg!(h[min]);
        
            h[min] -= a[mins][min] * minc;
            c[mins] -= minc;
            if c[mins] == 0 {
                available.remove(mini);
            }

            if h[min] <= 0 {
                //dbg!(mins, min, h[min], a[mins][min]);
                available.push(min);
                last -= 1;
                //dbg!("a", min);
                continue;
            }
        } else {
            let min = h.iter().enumerate().filter(|(_, &x)| x > 0).min_by_key(|(_, &x)| x).unwrap().0 as usize;
            for _ in 0..h[min] {
                println!("-1 {}", min);
            }
        
            h[min] = 0;
            available.push(min);
            last -= 1;
            //dbg!("b", min);
        }
    }
}
