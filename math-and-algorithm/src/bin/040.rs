use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize;n-1],
        m: usize,
        b: [isize;m]
    }

    let mut s = vec![0];
    for i in a.iter().enumerate() {
        s.push(s[i.0] + i.1);
    }

    let mut temp = b[0];
    let mut an = 0;
    for i in b[1..].iter() {
        an += s[(temp.max(*i) - 1) as usize] - s[(temp.min(*i) - 1) as usize];
        temp = *i;
    }
    
    println!("{}", an);
}
