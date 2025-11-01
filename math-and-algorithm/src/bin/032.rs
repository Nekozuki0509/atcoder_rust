use proconio::input;

fn main() {
    input! {
        n: usize,
        x: isize,
        mut a: [isize;n]
    }

    a.sort();

    let (mut left, mut right) = (0, n-1);
    while left <= right && right < a.len() {
        let mid: usize = (left + right) / 2;
        if a[mid] == x {
            println!("Yes");
            return;
        } else if a[mid] > x {
            right = (mid as isize - 1) as usize;
        } else {
            left = mid + 1;
        }
    }
    
    println!("No");
}
