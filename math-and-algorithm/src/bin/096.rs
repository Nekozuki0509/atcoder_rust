use proconio::input;

fn main() {
    input! {
        n: usize,
        t: [usize;n]
    }

    let sum = t.iter().sum::<usize>();
    let mut dp = vec![vec![false; sum + 1]; n + 1];
    dp[0][0] = true;

    for (i, &v) in t.iter().enumerate() {
        for (j, &w) in dp[i].clone().iter().enumerate() {
            if w {
                dp[i + 1][j] = true;
                dp[i + 1][j + v] = true;
            }
        }
    }

    let mut min = 2 << 60;
    for (i, &v) in dp[n].iter().enumerate() {
        if v && min > i.max(sum - i) {
            min = i.max(sum - i);
        }
    }

    println!("{}", min);
}
