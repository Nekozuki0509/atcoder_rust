use proconio::input;

fn main() {
    input! {
        n: usize,
        s: usize,
        a: [usize;n]

    }

    for i in 0..2u32.pow(n as u32) {
        let mut sum = 0;
        for j in 0..n {
            if i & 2u32.pow((j) as u32) != 0 {
                sum += a[j];
            }
        }

        if sum == s {
            println!("Yes");
            return;
        }
    }
    
    println!("No");
}
