use proconio::input;

fn main() {
    input! {
        (a, b): (usize, usize),
    }

    let mut an = 0;
    for i in 1..=b {
        if (b / i) >= 1 + ((a + i - 1) / i) {
            an = i;
        }
    }
    
    println!("{}", an);
}
