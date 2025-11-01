use proconio::input;

fn main() {
    input! {
        p1: (isize, isize),
        p2: (isize, isize),
        p3: (isize, isize),
        p4: (isize, isize)
    }

    if p1.0.max(p2.0) >= p3.0.min(p4.0) && p1.0.min(p2.0) <= p3.0.max(p4.0) && p1.1.max(p2.1) >= p3.1.min(p4.1) && p1.1.min(p2.1) <= p3.1.max(p4.1) {

        let v31 = (p1.0 - p3.0, p1.1 - p3.1);
        let v32 = (p2.0 - p3.0, p2.1 - p3.1);
        let v41 = (p1.0 - p4.0, p1.1 - p4.1);
        let v42 = (p2.0 - p4.0, p2.1 - p4.1);
        let v13 = (p3.0 - p1.0, p3.1 - p1.1);
        let v14 = (p4.0 - p1.0, p4.1 - p1.1);
        let v23 = (p3.0 - p2.0, p3.1 - p2.1);
        let v24 = (p4.0 - p2.0, p4.1 - p2.1);
        
        let o31_32 = v31.0 * v32.1 - v31.1 * v32.0;
        let o41_42 = v41.0 * v42.1 - v41.1 * v42.0;
        let o13_14 = v13.0 * v14.1 - v13.1 * v14.0;
        let o23_24 = v23.0 * v24.1 - v23.1 * v24.0;

        if ((o31_32 <= 0 && o41_42 >= 0) || (o31_32 >= 0 && o41_42 <= 0)) && ((o13_14 <= 0 && o23_24 >= 0) || (o13_14 >= 0 && o23_24 <= 0)) {
            println!("Yes");
        } else {
            println!("No");
        }
    } else {
        println!("No");
    }
}
