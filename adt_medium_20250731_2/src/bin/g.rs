use amplify::confinement::Collection;
use num::abs;
#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

enum Way {
    UP,
    DOWN,
    RIGHT,
    LEFT
}

const ways: Vec<Way> = vec![Way::UP, Way::DOWN, Way::RIGHT, Way::LEFT];

#[fastout]
fn main() {
    input! {
        (n, m): (usize, usize),
        s: [Chars;n]
    }

    let mut start = 0usize;
    let mut set = vec![];
    for (i, r) in s.iter().enumerate() {
        for (j, c) in r.iter().enumerate() {
            if c.eq(&'.') {
                set.push((i + 1, j + 1));
                if i == 1 && j == 1 {
                    start = set.len() - 1;
                }
            }            
        }
    }

    let mut d = vec![vec![]; set.len()];
    for (i, v) in set.iter().enumerate() {
        d[i].append(&mut set.iter().enumerate()
                            .filter(|(i, x)| abs(x.0 as isize - v.0 as isize) + abs(x.1 as isize - v.1 as isize) == 1)
                            .map(|(i, x)| i)
                            .collect::<Vec<usize>>()
                        );
    }

    let mut visited = vec![false;set.len()];
    for i in 0..4 {
        dfs(start, &d, &mut visited, ways[i]);
    }

}

fn dfs(pos: usize, g: &Vec<Vec<usize>>, visited: &mut Vec<bool>, way: Way) {
    visited[pos] = true;

    let t = &g[pos];
    for i in t {
        if visited[*i] == false {
            for j in 0..4 {
                dfs(*i, g, &mut visited, ways[j]);
            }
        }
    }
}