use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize;n]
    }

    let mut lr = a[0].clone();
    let mut i_temp;
    let mut lr_temp;
    for i in a.clone()[1..].iter_mut() {
        i_temp = *i;
        lr_temp = lr;
        while *i != 0 && lr != 0 {
            if *i >= lr {
                *i %= lr;
            } else {
                lr %= *i;
            }
        }

        lr = lr_temp / lr.max(*i) * i_temp;
    }
    
    println!("{}", lr);
}
