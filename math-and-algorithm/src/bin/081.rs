use proconio::input;

fn main() {
    input! {
        mut n: usize,
    }

    let mut an = 0;
    an += n / 10000;
    n %= 10000;
    an += n / 5000;
    n %= 5000;
    an += n / 1000;
    
    println!("{}", an);
}
