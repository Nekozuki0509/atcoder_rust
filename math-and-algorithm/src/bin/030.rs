use proconio::input;

fn main() {
    input! {
        n: usize,
        w: usize,
        mut s: [(usize, isize);n]
    }

    s.push((0, 0));
    s = s.to_vec();
    s.reverse();

    let mut dp = vec![];
    dp.push(vec![0]);
    for _ in 1..=w {
        dp[0].push(-(2isize.pow(60)));
        dp.push(vec![]);
    }

    for i in 1..=n {
        for j in 0..=w {
            let p = if j < s[i].0 {
                dp[i-1][j]
            } else {
                dp[i-1][j].max(dp[i-1][j-s[i].0]+s[i].1)
            };

            dp[i].push(p);
        }
    }

    let mut an = 0;
    for i in 0..=w {
        an = an.max(dp[n][i]);
    }
    
    println!("{}", an);
}
