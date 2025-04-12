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

    let mut visited = vec![false; n + 1];

    dfs(1, &g, &mut visited);
    
    println!("The graph is{} connected.", if visited[1..].contains(&false) {" not"} else {""});
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
