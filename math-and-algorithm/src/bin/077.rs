use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut xset = vec![];
    let mut yset = vec![];
    for _ in 0..n {
        input! {x: isize, y: isize}
        xset.push(x);
        yset.push(y);
    }

    xset.sort();
    yset.sort();

    let mut an = 0;
    for (i, (&x, y)) in xset.iter().zip(yset).enumerate() {
        //dbg!(x, (2*(i as isize)+1-(n as isize)), y, (2*(i as isize)+1-(n as isize)));
        an += x * (2*(i as isize)+1-(n as isize)) + y * (2*(i as isize)+1-(n as isize));
    }
    
    println!("{}", an);
}
