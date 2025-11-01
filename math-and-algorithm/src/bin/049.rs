use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut a = vec![0usize, 1, 1];

    for i in 3..=n {
        a.push((a[i-1] + a[i-2])  % 1000000007);
    }
    
    println!("{}", a[n] % 1000000007);
}
