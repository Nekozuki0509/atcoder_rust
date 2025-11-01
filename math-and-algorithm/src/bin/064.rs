use proconio::input;

fn main() {
    input! {
        (n, k): (usize, usize),
        a: [usize;n]
    }

    let sum = a.iter().sum::<usize>();
    let ans = if sum > k {
        "No"
    } else if (k - sum) % 2 == 0 {
        "Yes"
    } else {
        "No"
    };
    
    println!("{}", ans);
}
