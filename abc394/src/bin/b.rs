use proconio::input;

fn main() {
    input! {
        n: u32,
        mut s:[String;n],
    }

    s.sort_by(|a, b| a.chars().count().cmp(&b.chars().count()));

    let mut o = String::new();

    s.iter().for_each(|c| o.push_str(&c));

    println!("{}", o);
}