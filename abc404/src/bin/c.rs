#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize
    }

    let mut g = vec![vec![]; n + 1];
    for i in 1..=m {
        input! { v: usize, w: usize}
        g[v].push(w);
        g[w].push(v);
    }
    for i in g[1..].iter() {
        //println!("i={}", i.len());
        if i.len() != 2 {
            println!("No");
            return;
        }
    }

    let mut visited = vec![false; n + 1];

    dfs(1, &g, &mut visited);
    
    println!("{}", if visited[1..].contains(&false) {"No"} else {"Yes"});
}

fn dfs(pos: usize, g: &Vec<Vec<usize>>, visited: &mut Vec<bool>) {
    visited[pos] = true;

    let t = &g[pos];
    for i in t {
        if visited[*i] == false {
            dfs(*i, g, visited);
        }
    }
}