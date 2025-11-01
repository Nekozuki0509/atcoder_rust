use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
    }

    let mut g = vec![vec![]; n];
    for _ in 1..n {
        input! {a: Usize1, b: Usize1}
        g[a].push(b);
        g[b].push(a);
    }

    let mut visited = vec![false; n];
    let mut dp = vec![0; n];
    dfs(0, &mut visited, &g, &mut dp);
    let mut ans = 0;
    for i in 1..n {
        ans += dp[i] * (n - dp[i]);
    }

    println!("{}", ans);
}

fn dfs(pos: usize, visited: &mut Vec<bool>, g: &Vec<Vec<usize>>, dp: &mut Vec<usize>) {
    visited[pos] = true;
    dp[pos] = 1;
    for &i in &g[pos] {
        if !visited[i] {
            dfs(i, visited, g, dp);
            dp[pos] += dp[i];
        }
    }
}
