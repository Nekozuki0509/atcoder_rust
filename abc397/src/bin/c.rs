use proconio::input;

fn main() {
    input! {
        n: u32,
        mut a:[u32;n],
    }

    let mut an = 0;
    let mut b:Vec<u32> = vec![];
    for _ in 1..=n-1 {
        if a.is_empty() {
            break;
        } 

        let temp = a.pop().unwrap();

        match b.iter().find(|&x| x == &temp) {
            Some(_) => continue,
            _ => {}
        }
        if !b.is_empty() && b.last().unwrap() == a.last().unwrap() {
            b.push(temp);
            continue;
        }

        b.push(a.pop().unwrap());

        let mut c = a.clone().to_vec();
        c.sort();
        c.dedup();
        let temp = c.iter().count();

        let mut c = b.clone().to_vec();
        c.sort();
        c.dedup();

        let sum = temp + c.iter().count();
        if sum > an {
            an = sum;
        }
    }
    
    println!("{}", an);
}
