use std::{collections::BTreeMap, io::{stdin, stdout, BufReader, Write}};

use itertools::Itertools;
use num::abs;
#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}, source::line::LineSource};

//#[fastout]
fn main() {
    let stdin = stdin();
    let mut source = LineSource::new(BufReader::new(stdin.lock()));
    //dbg!("start...");

    input! {
        from &mut source,
        mut n: usize,
        m: usize,
        l: usize,
        u: usize
    }

    //dbg!("inputed");

    let d = u - l;

    let mut vec = vec![];
    let mut basevec = vec![(l, m)];
    for _ in 0..m {
        vec.push(l);
    }
    n -= m;
    let mut now = 2;
    loop {
        let next = 50 * (1 - 1 / now);
        let nextv = d / now;
        if next > n {
            basevec.push((nextv, n));
            for _ in 0..n {
                vec.push(nextv);
            }

            break;
        }

        basevec.push((nextv, next));
        for _ in 0..next {
            vec.push(nextv);
        }

        n -= next;
        now *= 2;
    }

    print!("{}", vec.iter().map(|x| x.to_string()).join(" "));
    println!();
    stdout().flush().unwrap();

    //dbg!("printed");

    input! {
        from &mut source,
        b: [isize;m]
    }

    //dbg!("now");

    let mut out = vec![vec![];basevec.len()];
    let mut sums = vec![];
    for (i, &v) in b.iter().enumerate() {
        //dbg!(i);
        let mut di = -1;
        let mut used = vec![];
        let mut msum = 0;
        for j in 1usize..(1 << basevec.len()) {
            let mut sum = 0;
            let mut nowused = vec![false;basevec.len()];
            for k in 0..basevec.len() {
                if j & (1 << k) != 0 && basevec[k].1 != 0 {
                    sum += vec[k];
                    nowused[k] = true;
                }
            }
            
            if di == -1 || abs(sum as isize - v) < di {
                di = abs(sum as isize - v);
                used = nowused.clone();
                msum = sum;
            }
        }

        for (j, &v) in used.iter().enumerate() {
            if v {
                basevec[j].1 -= 1;
                out[j].push(i+1);
                sums.push(msum);
            }
        }
    }

    let mut improvevec = vec![true;m];
    for (i, &v) in sums.iter().enumerate() {
        if b[i] <= v as isize {
            improvevec[i] = false;
        }
    }
    while improvevec.iter().any(|&x| x) && basevec.iter().any(|&(_, v)| v != 0) {
        let cimprove = improvevec.iter().enumerate().filter_map(|(i, &v)| if v {Some(i)} else {None}).collect_vec();
        for i in cimprove {
            let mut flag = true;
            let cbase = basevec.clone();
            for (j, &(v, l)) in cbase.iter().enumerate() {
                if l == 0 {
                    continue;
                }

                if abs(sums[i] as isize - b[i]) > abs(sums[i] as isize + j as isize - b[i]) {
                    sums[i] += v;
                    basevec[j].1 -= 1;
                    out[j].push(i+1);
                    flag = false;
                }
            }

            if flag {
                improvevec[i] = false;
            }
        }
    }

    let mut last = vec![];
    for (o, (_, b)) in out.iter().zip(basevec) {
        for &i in o {
            last.push(i);
        }
        for _ in 0..b {
            last.push(0);
        }
    } 

    print!("{}", last.iter().map(|x| x.to_string()).join(" "));
    stdout().flush().unwrap();
}
