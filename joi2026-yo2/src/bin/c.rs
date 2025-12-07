#![allow(unused_imports, dead_code, non_snake_case)]
#![allow(unused_variables, unused_assignments)]
use itertools::Itertools;
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1, Isize1}};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: Chars
    }

    let mut ans = vec![];
    let mut last = 'a';
    let mut oiflag = false;
    let mut jnum = 0;
    let mut oinum = 0;
    //let mut pushed = false;
    let mut lastpushed = false;
    for &i in s.iter().rev() {
        //pushed = false;
        lastpushed = false;
        // if flag {
        //     last = i;
        //     flag = false;
        //     continue;
        // }

        match i {
            'O' => {
                if last == 'I' {
                    oinum += 1;
                    oiflag = true;
                } else {
                    ans.push("J".repeat(jnum));
                    ans.push("OI".repeat(oinum));
                    ans.push("O".to_string());
                    jnum = 0;
                    oinum = 0;
                    //pushed = true;
                    oiflag = false;
                }
                
                lastpushed = true;
                last = 'O';
            },
            'I' => {
                if (last == 'I') || (last == 'O' && !oiflag) {
                    ans.push("J".repeat(jnum));
                    ans.push("OI".repeat(oinum));
                    if last == 'I' {
                        ans.push(last.to_string());
                    }
                    jnum = 0;
                    oinum = 0;
                    //pushed = true;
                    //lastpushed = true;
                }
                oiflag = false;
                last = 'I';
            },
            _ => {
                if last == 'I' {
                    ans.push("J".repeat(jnum));
                    ans.push("OI".repeat(oinum));
                    //ans.push(last.to_string());
                    if last == 'I' {
                        ans.push(last.to_string());
                    }
                    jnum = 0;
                    oinum = 0;
                    //pushed = true;
                }
                jnum += 1;
                last = 'J';
                lastpushed = true;
                oiflag = false;
            }
        }
    }

    if jnum > 0 {
        ans.push("J".repeat(jnum));
    }
    if oinum > 0 {
        ans.push("OI".repeat(oinum));
    } 
    
    if !lastpushed {
        ans.push(last.to_string());
    }

    println!("{}", ans.iter().rev().join(""))
}
