use proconio::input;

fn main() {
    input! {
        n: u32,
        a: [u32;n],
        b: [u32;n]
    }
    
    println!("{}", a.iter().sum::<u32>() as f32 / n as f32 + b.iter().sum::<u32>() as f32 / n as f32);
}
