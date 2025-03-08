use proconio::input;

fn main() {
    input! {
        mut s: String,
    }

    while s.contains("()") || s.contains("[]") || s.contains("<>") {
        s = s.replace("()", "").replace("[]", "").replace("<>", "");
    }

    loop {
        if s.contains("()") {
            s = s.replace("()", "");
        }

        if s.contains("") {
            s = s.replace("()", "");
        }

        if s.contains("()") {
            s = s.replace("()", "");
        }
    }

    if "".eq(&s) {
        println!("Yes");
    } else {
        println!("No");
    }
}