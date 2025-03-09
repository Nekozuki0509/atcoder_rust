use proconio::input;

fn main() {
    input! {
        n: u64,
        m: u64,
        mut b:[i64; n],
        mut w:[i64; m],
    }

    w = w.iter().filter(|&x| x > &0).cloned().collect();

    b.sort();
    b.reverse();

    //println!("{:?}", b);

    w.sort();
    w.reverse();

    //println!("{:?}", w);

    let mut temp = 0;

    for i in 0..w.len() {
        if b.len() - 1 < i {break;} 

        if (b[i] > 0 && w[i] > 0) || b[i] + w[i] > 0   {
            //println!("fis");
            temp += b[i];
            temp += w[i];
            //println!("{}", temp)
        }
    }

    for i in (w.len())..(b.len()) {
        //println!("i={}", i);
        if b[i] > 0 {
            temp += b[i];
            //println!("temp={}", temp);
        } else {
            break;
        }
    }
    
    println!("{}", temp);
}
