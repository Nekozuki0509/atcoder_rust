use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, k): (usize, usize),
        a: [Usize1;n]
    }

    let mut visited = vec![false;n];
    let mut vec = vec![0];
    let mut now = 0;
    visited[0] = true;

    loop {
        if vec.len() - 1 == k {
            println!("{}", now + 1);
            return;
        }
        
        now = a[now];

        if visited[now] {
            let i = vec.iter().position(|&x| x == now).unwrap();
            println!("{}", vec[(k-i)%(vec.len()-i)+i] + 1);
            return;
        }

        vec.push(now);
        visited[now] = true;
    }
}
