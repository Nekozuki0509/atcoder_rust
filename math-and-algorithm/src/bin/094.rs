use proconio::input;

fn main() {
    input! {
        n: usize,
        b: [usize;n-1]
    }

    let mut an = b[0] + b[n - 2];
    for i in b.windows(2) {
        an += i[0].min(i[1]);
    }

    println!("{}", an);
}
