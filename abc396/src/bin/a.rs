use proconio::input;

fn main() {
    input! {
        n: u32,
        a:[i32;n],
    }
    
    let mut temp1 = 0;
    let mut temp2 = 0;

    for i in a {
        if temp1 == 0 {
            temp1 = i;
            continue;
        } else if temp2 == 0 {
            temp2 = i;
            continue;
        } else if i == temp2 && temp2 == temp1 {
            println!("Yes");
            std::process::exit(0);
        } else {
            temp1 = temp2;
            temp2 = i;
        }
    }

    println!("No");
}
