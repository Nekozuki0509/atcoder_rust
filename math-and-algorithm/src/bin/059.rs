use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let ans = match n % 4 {
        1 => 2,
        2 => 4,
        3 => 8,
        _ => 6
    };

    println!("{}", ans);
}
