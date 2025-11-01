use proconio::input;

fn main() {
    input! {
        mut a: usize,
        b: usize
    }

    let mut an = 1usize;
    for i in 0..30 {
        if (b & (1 << i)) != 0 {
            an *= a;
            an %= 1000000007;
        }
        a = a.pow(2);
        a %= 1000000007;
    }
    
    println!("{}", an);
}
