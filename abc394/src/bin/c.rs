use proconio::input;

fn main() {
    input! {
        mut s: String,
    }

    while s.contains("WA") {
        s = s.replace("WA", "AC");
    }

    println!("{}", s);
}