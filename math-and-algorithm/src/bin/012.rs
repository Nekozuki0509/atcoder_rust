use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    for i in 2..=n {
        if i * i > n {
            break;
        }

        if n % i == 0 {
            println!("No");
            return;
        }
    }

    println!("Yes");
}