use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize;n]
    }

    let mut dp = vec![0, a[0], a[1]];
    for i in a[2..].iter().enumerate() {
        dp.push((dp[i.0+1]+a[i.0+2]).max(dp[i.0]+a[i.0+2])); 
    }
    
    println!("{}", dp[n].max(dp[n-1]));
}
