pub fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

pub fn gcd_list(list: &[u64]) -> u64 {
    list.iter().fold(list[0], |a, &b| gcd(a, b))
}

pub fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}