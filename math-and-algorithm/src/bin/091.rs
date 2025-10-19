use proconio::input;

fn main() {
    input! {
        (n, x): (usize, usize)
    }

    let mut an = 0;
    for a in 1..=n {
        for b in a + 1..=n {
            if x > a + 2 * b && x - a - b <= n {
                an += 1;
            }
        }
    }

    println!("{}", an);
}
