use proconio::input;

fn main() {
    input! {
        s1: String,
        s2: String,
    }

    let b1 = "sick".eq(&s1);
    let b2 = "sick".eq(&s2);

    let a;

    if b1 && b2 {a = "1";}
    else if b1 && !b2 {a = "2";}
    else if !b1 && b2 {a = "3";}
    else {a = "4";}

    println!("{}", a);
}
