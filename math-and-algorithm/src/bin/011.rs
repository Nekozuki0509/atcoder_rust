use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut set = vec![];

    for i in 2..=n {
        if is_prime(&set, i) {
            set.push(i);
        }
    }

    println!("{}", set.iter().fold(String::new(), |acc, &x| format!("{}{} ", acc, x)).trim_end());
}

fn is_prime(set:&Vec<usize>, n: usize) -> bool {
    for num in set {
        if n % num == 0 {
            return false;
        }
    }
    true
}
