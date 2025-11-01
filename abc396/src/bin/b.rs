use proconio::input;

fn main() {
    input! {
        q: u32,
    }

    let mut a = vec![];

    for _ in 0..q {
        input! {
            c: u32
        }

        if c == 1 {
            input! {
                n: String
            }
            a.push(format!("1 {}", n));
        } else {
            a.push(String::from("2"));
        }
    }

    //println!("{:?}", a);

    let mut cards = vec![0; 100];

    for s in a {
        let parts = s.split(" ");
        if parts.clone().count() == 1 as usize {
            println!("{}", cards.last().unwrap());
            cards.remove(cards.len() - 1);
        } else {
            cards.push(parts.last().unwrap().parse().unwrap());
            //println!("{:?}", cards);
        }
    }
}
