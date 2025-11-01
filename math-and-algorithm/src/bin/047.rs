use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        (n, m): (usize, usize),
    }

    let mut g = vec![vec![]; n + 1];
    for _ in 0..m {
        input! {(v, w): (usize, usize)}

        g[v].push(w);
        g[w].push(v);
    }

    let mut d: Vec<isize> = vec![-1; n + 1];
    d[0] = 0;

    let mut q = VecDeque::new();

    while d.contains(&-1) {
        let m1 = d.iter().position(|x| x == &-1).unwrap();
        q.push_back(m1);
        d[m1] = 0;

        while !q.is_empty() {
            let pos = q.pop_front().unwrap();

            for i in &g[pos] {
                if d[*i] == -1 {
                    d[*i] = if d[pos] == 0 {1} else {0};
                    q.push_back(*i);
                } else if d[*i] == d[pos] {
                    println!("No");
                    return;
                }
            }
        }
    }

    //println!("{:?}", d);

    println!("Yes");
}
