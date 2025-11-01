fn main() {
    println!("17 16");

    let mut now = 5;
    for i in 2..=5 {
        println!("1 {}", i);
        for _ in 0..3 {
            now += 1;
            println!("{} {}", i, now);
        }
    }
}
