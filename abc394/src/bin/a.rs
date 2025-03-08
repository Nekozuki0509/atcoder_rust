use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }

    let mut o: String = String::new();
    s.iter().for_each(|m| if '2'.eq(m) { o.push('2'); });

    println!("{}", o);
}
