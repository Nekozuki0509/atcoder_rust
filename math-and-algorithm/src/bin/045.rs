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

    let mut an = 0;
    for (i, v) in g.iter().enumerate() {
        if v.iter().filter(|&x| x < &i).count() == 1 {
            an += 1;
        }
    }
    
    println!("{}", an);
}
